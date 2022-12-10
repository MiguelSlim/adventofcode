import fs from "fs";

const data = fs.readFileSync("../inputs/day_7_part_2.txt", {
  encoding: "utf8",
  flag: "r",
});

const fileSystem = { type: "dir", name: "/", contents: [] };
let pushToDir = false;
let curDir = fileSystem;

for (const line of data.split("\n")) {
  if (line[0] === "$") {
    const [_, command, dirName] = line.split(" ");

    if (command === "cd") {
      if (dirName === "..") curDir = getParentDirectory(curDir, fileSystem);
      else curDir = getChildDirectory(curDir, dirName);
    }

    pushToDir = command === "ls" ? true : false;
  } else {
    if (!pushToDir) continue;

    const [firstArg, name] = line.split(" ");
    if (firstArg === "dir") {
      const newDir = createEmptyDir(name);
      curDir.contents.push(newDir);
    } else {
      const newFile = createFile(name, firstArg);
      curDir.contents.push(newFile);
    }
  }
}

function getParentDirectory(dir, fileSystem) {
  let prevDir = null;
  for (const item of fileSystem.contents) {
    if (item.type === "dir") {
      if (item === dir) prevDir = fileSystem;
      else prevDir = getParentDirectory(dir, item);
      if (prevDir) break;
    }
  }

  return prevDir;
}

function getChildDirectory(dir, name) {
  return dir.contents.find((item) => item.name === name);
}

function createEmptyDir(name) {
  return { type: "dir", name, contents: [] };
}

function createFile(name, size) {
  return { type: "file", name, size: Number(size) };
}

function recursivelyGetDirSizes(fileSystem) {
  const dirs = [];
  const dirSize = getDirSize(fileSystem);
  dirs.push(dirSize);

  for (const item of fileSystem.contents) {
    if (item.type === "dir") dirs.push(...recursivelyGetDirSizes(item));
  }

  return dirs;
}

function getDirSize(d) {
  return d.contents.reduce((a, c) => (a += c.size || getDirSize(c)), 0);
}

const dirSizes = recursivelyGetDirSizes(fileSystem);

/** Part 1 */
const MAX_SIZE = 100_000;

const resultPart1 = dirSizes
  .filter((dir) => dir < MAX_SIZE)
  .reduce((a, c) => (a += c), 0);

console.log(resultPart1);

/** Part 2 */
const SPACE_AVAILABLE = 70_000_000;
const MIN_UNUSED_SPACE = 30_000_000;

const unusedSpace = SPACE_AVAILABLE - getDirSize(fileSystem);
const spaceToFreeUp = MIN_UNUSED_SPACE - unusedSpace;

const resultPart2 = dirSizes
  .filter((dir) => dir >= spaceToFreeUp)
  .sort((a, b) => a - b)[0];

console.log(resultPart2);
