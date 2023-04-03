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
  ])("Valid app name - (%s)", async (packageName) => {
    const wasm = await bindings.polyvalid();
    expect(wasm.isAppNameValid(packageName).isValid).toBe(true);
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
  ])("Invalid app name - (%s)", async (packageName) => {
    const wasm = await bindings.polyvalid();
    expect(wasm.isAppNameValid(packageName).isValid).toBe(false);
  });
});
