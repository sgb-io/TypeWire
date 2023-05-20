#!/usr/bin/env node

const { execSync } = require("node:child_process");
const path = require("node:path");
const fs = require("node:fs");

const platform = process.platform;
const architecture = process.arch;

function getBinaryPath() {
  const targetDirectory = path.join(__dirname, "binaries");

  switch (platform) {
    case "win32":
      if (architecture === "x64") {
        return path.join(targetDirectory, "x86_64-pc-windows-msvc", "fta.exe");
      } else if (architecture === "arm64") {
        return path.join(targetDirectory, "aarch64-pc-windows-msvc", "fta.exe");
      }
    case "darwin":
      if (architecture === "x64") {
        return path.join(targetDirectory, "x86_64-apple-darwin", "fta");
      } else if (architecture === "arm64") {
        return path.join(targetDirectory, "aarch64-apple-darwin", "fta");
      }
    case "linux":
      if (architecture === "x64") {
        return path.join(targetDirectory, "x86_64-unknown-linux-gnu", "fta");
      } else if (architecture === "arm64") {
        return path.join(targetDirectory, "aarch64-unknown-linux-gnu", "fta");
      } else if (architecture === "arm") {
        return path.join(
          targetDirectory,
          "armv7-unknown-linux-gnueabihf",
          "fta"
        );
      }
      break;
    default:
      throw new Error("Unsupported platform: " + platform);
  }

  throw new Error("Binary not found for the current platform");
}

function setUnixPerms(binaryPath) {
  if (platform === "darwin" || platform === "linux") {
    try {
      fs.chmodSync(binaryPath, "755");
    } catch (e) {
      console.warn("Could not chmod fta binary: ", e);
    }
  }
}

// Run the binary from code
// We build arguments that get sent to the binary
function runFta(project, options) {
  const binaryPath = getBinaryPath();
  const binaryArgs = options.json ? "--json" : "";
  setUnixPerms(binaryPath);
  const result = execSync(`${binaryPath} ${project} ${binaryArgs}`);
  return result.toString();
}

// Run the binary directly if executed as a standalone script
// Arguments are directly forwarded to the binary
if (require.main === module) {
  const args = process.argv.slice(2); // Exclude the first two arguments (node binary and script file)
  const binaryPath = getBinaryPath();
  const binaryArgs = args.join(" ");
  setUnixPerms(binaryPath);
  const result = execSync(`${binaryPath} ${binaryArgs}`);
  console.log(result.toString());
}

module.exports.runFta = runFta;
