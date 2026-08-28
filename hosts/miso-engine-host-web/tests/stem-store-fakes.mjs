export class FakeOpfsBackend {
  quotaFailures = 0

  constructor(options = {}) {
    this.root = directoryNode()
    this.quota = options.quota ?? Number.MAX_SAFE_INTEGER
    this.reportedQuota = options.reportedQuota ?? this.quota
    this.moveSupported = options.moveSupported ?? true
    this.moveError = options.moveError
    this.writeFailure = options.writeFailure
  }

  get usage() {
    return nodeBytes(this.root)
  }

  storage() {
    return {
      getDirectory: async () => new FakeDirectoryHandle(this, this.root),
      estimate: async () => ({ quota: this.reportedQuota, usage: this.usage }),
    }
  }

  bytes(path) {
    const node = findNode(this.root, path)
    if (node.kind !== "file") throw new TypeError(`${path} is not a file`)
    return node.bytes
  }

  setBytes(path, bytes) {
    const node = findNode(this.root, path)
    if (node.kind !== "file") throw new TypeError(`${path} is not a file`)
    node.bytes = new Uint8Array(bytes)
    node.lastModified = Date.now()
  }

  remove(path) {
    const names = path.split("/").filter(Boolean)
    const name = names.pop()
    if (name === undefined) throw new TypeError("fake path must name an entry")
    const parent = findNode(this.root, names.join("/"))
    if (parent.kind !== "directory") throw new TypeError(`${path} has no directory parent`)
    if (!parent.entries.delete(name)) throw domError("NotFoundError", `${path} is absent`)
  }

  has(path) {
    try {
      findNode(this.root, path)
      return true
    } catch (error) {
      if (error?.name === "NotFoundError") return false
      throw error
    }
  }

  names(path) {
    const node = findNode(this.root, path)
    if (node.kind !== "directory") throw new TypeError(`${path} is not a directory`)
    return Array.from(node.entries.keys()).sort()
  }
}

class FakeDirectoryHandle {
  kind = "directory"

  constructor(backend, node) {
    this.backend = backend
    this.node = node
  }

  async getDirectoryHandle(name, options = {}) {
    let child = this.node.entries.get(name)
    if (child === undefined && options.create) {
      child = directoryNode()
      this.node.entries.set(name, child)
    }
    if (child === undefined) throw domError("NotFoundError", `${name} is absent`)
    if (child.kind !== "directory") throw domError("TypeMismatchError", `${name} is a file`)
    return new FakeDirectoryHandle(this.backend, child)
  }

  async getFileHandle(name, options = {}) {
    let child = this.node.entries.get(name)
    if (child === undefined && options.create) {
      child = fileNode()
      this.node.entries.set(name, child)
    }
    if (child === undefined) throw domError("NotFoundError", `${name} is absent`)
    if (child.kind !== "file") throw domError("TypeMismatchError", `${name} is a directory`)
    return new FakeFileHandle(this.backend, this.node, name, child)
  }

  async removeEntry(name, options = {}) {
    const child = this.node.entries.get(name)
    if (child === undefined) throw domError("NotFoundError", `${name} is absent`)
    if (child.kind === "directory" && child.entries.size !== 0 && !options.recursive) {
      throw domError("InvalidModificationError", `${name} is not empty`)
    }
    this.node.entries.delete(name)
  }

  async *entries() {
    for (const [name, node] of Array.from(this.node.entries.entries())) {
      yield [
        name,
        node.kind === "file"
          ? new FakeFileHandle(this.backend, this.node, name, node)
          : new FakeDirectoryHandle(this.backend, node),
      ]
    }
  }
}

class FakeFileHandle {
  kind = "file"

  constructor(backend, parent, name, node) {
    this.backend = backend
    this.parent = parent
    this.name = name
    this.node = node
    if (!backend.moveSupported) this.move = undefined
  }

  async getFile() {
    const blob = new Blob([this.node.bytes])
    Object.defineProperty(blob, "lastModified", { value: this.node.lastModified })
    return blob
  }

  async createWritable() {
    const chunks = []
    let closed = false
    const backend = this.backend
    const node = this.node
    return {
      async write(value) {
        if (closed) throw new Error("fake writable is closed")
        if (backend.writeFailure?.({ handle: this, value, chunks })) {
          backend.quotaFailures += 1
          throw domError("QuotaExceededError", "fake quota exhausted")
        }
        if (typeof value === "string") chunks.push(new TextEncoder().encode(value))
        else if (value instanceof Blob) chunks.push(new Uint8Array(await value.arrayBuffer()))
        else if (value instanceof Uint8Array) chunks.push(new Uint8Array(value))
        else if (value instanceof ArrayBuffer) chunks.push(new Uint8Array(value))
        else throw new TypeError("unsupported fake writable value")
      },
      async close() {
        if (closed) return
        closed = true
        const bytes = concatenate(chunks)
        const replaced = node.bytes.byteLength
        if (backend.usage - replaced + bytes.byteLength > backend.quota) {
          backend.quotaFailures += 1
          throw domError("QuotaExceededError", "fake quota exhausted")
        }
        node.bytes = bytes
        node.lastModified = Date.now()
      },
      async abort() {
        closed = true
      },
    }
  }

  async move(destinationOrName, maybeName) {
    if (this.backend.moveError !== undefined) throw this.backend.moveError
    let destination
    let name
    if (typeof destinationOrName === "string") {
      destination = this.parent
      name = destinationOrName
    } else {
      destination = destinationOrName.node
      name = maybeName
    }
    if (destination.entries.has(name)) {
      throw domError("NoModificationAllowedError", `${name} exists`)
    }
    destination.entries.set(name, this.node)
    this.parent.entries.delete(this.name)
    this.parent = destination
    this.name = name
  }
}

export class FakeLockManager {
  #tails = new Map()
  #held = new Map()
  #pending = new Map()
  requests = []

  async request(name, _options, work) {
    this.requests.push(name)
    const previous = this.#tails.get(name) ?? Promise.resolve()
    this.#pending.set(name, (this.#pending.get(name) ?? 0) + 1)
    let release
    const gate = new Promise((resolve) => {
      release = resolve
    })
    const tail = previous.then(() => gate)
    this.#tails.set(name, tail)
    await previous
    decrement(this.#pending, name)
    this.#held.set(name, (this.#held.get(name) ?? 0) + 1)
    try {
      return await work({ name, mode: "exclusive" })
    } finally {
      decrement(this.#held, name)
      release()
      if (this.#tails.get(name) === tail) this.#tails.delete(name)
    }
  }

  async query() {
    return {
      held: Array.from(this.#held.keys(), (name) => ({ name, mode: "exclusive" })),
      pending: Array.from(this.#pending.keys(), (name) => ({ name, mode: "exclusive" })),
    }
  }
}

function directoryNode() {
  return { kind: "directory", entries: new Map() }
}

function fileNode() {
  return { kind: "file", bytes: new Uint8Array(), lastModified: Date.now() }
}

function findNode(root, path) {
  let node = root
  for (const name of path.split("/").filter(Boolean)) {
    if (node.kind !== "directory") throw new TypeError(`${name} crosses a file`)
    node = node.entries.get(name)
    if (node === undefined) throw domError("NotFoundError", `${path} is absent`)
  }
  return node
}

function nodeBytes(node) {
  if (node.kind === "file") return node.bytes.byteLength
  let bytes = 0
  for (const child of node.entries.values()) bytes += nodeBytes(child)
  return bytes
}

function concatenate(chunks) {
  const bytes = new Uint8Array(
    chunks.reduce((total, chunk) => total + chunk.byteLength, 0)
  )
  let offset = 0
  for (const chunk of chunks) {
    bytes.set(chunk, offset)
    offset += chunk.byteLength
  }
  return bytes
}

function domError(name, message) {
  const error = new Error(message)
  error.name = name
  return error
}

function decrement(map, key) {
  const value = map.get(key) - 1
  if (value === 0) map.delete(key)
  else map.set(key, value)
}
