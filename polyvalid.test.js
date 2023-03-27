const { bindings } = require("@ayys/polyvalid");
const { describe, test, expect } = require("@jest/globals");

describe("Polyvalid Package validator", () => {
  test.each([
    "ValidName1",
    "helloworld",
    "HELLOWORLD",
    "HelloworlD",
    "d981273",
    "Valid-Name",
    "hello-world",
    "hel__lo",
  ])("Valid Package - (%s)", async (packageName) => {
    const wasm = await bindings.polyvalid();
    expect(wasm.isNameValid(packageName)).toBe(true);
  });

  test.each([
    "1InvalidName",
    "Inva@lidName",
    "InvalidName!",
    " hello ",
    "-hello",
    "hello-",
    "-hello-",
    "_hello",
    "hello_",
    "--hello",
    "hello--",
    "hel--lo",
    "__hello",
    "hello__",
    "_hello_",
  ])("Invalid Package - (%s)", async (packageName) => {
    const wasm = await bindings.polyvalid();
    expect(wasm.isNameValid(packageName)).toBe(false);
  });
});
