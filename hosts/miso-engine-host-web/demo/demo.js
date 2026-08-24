const button = document.querySelector("#run");
const output = document.querySelector("#result");

button.addEventListener("click", async () => {
  button.disabled = true;
  output.textContent = "Running…";
  try {
    const { runQualification } = await import("/qualification/qualification.js");
    output.textContent = JSON.stringify(await runQualification(), null, 2);
  } catch (error) {
    output.textContent = `Qualification failed: ${error?.message ?? String(error)}`;
  } finally {
    button.disabled = false;
  }
});
