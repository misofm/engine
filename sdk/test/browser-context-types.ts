import { createEngine } from "../src/browser/index.ts";
const common = {
  sampleRate: 48000, state: "suspended", close: async () => {},
  audioWorklet: { addModule: async (_url: string) => {} },
};
async function injectedContexts() {
  const thin = await createEngine({ document: "", createContext: () => ({ ...common, marker: "thin" as const }) });
  const marker: "thin" = thin.context.marker;
  // @ts-expect-error A thin factory cannot promise Web Audio methods.
  thin.context.resume();
  const other = await createEngine({ document: "", createContext: () => ({ ...common, custom: () => 42 }) });
  const value: number = other.context.custom();
  // @ts-expect-error Independent factories do not acquire each other's properties.
  other.context.marker;
  return [marker, value];
}
void injectedContexts;
