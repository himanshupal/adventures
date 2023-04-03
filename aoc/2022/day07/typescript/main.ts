interface File {
  name: string;
  size: number;
}

interface Directory {
  folders: Array<Directory>;
  parent?: Directory;
  files: Array<File>;
  name: string;
  size: number;
}

const DISK_SIZE = 7e7;
const MIN_SPACE = 3e7;

let spaceLeft = DISK_SIZE;

const day07 = async () => {
  const file = await Deno.readTextFile("../input.txt");

  let root: Directory = {
    folders: [],
    files: [],
    name: "/",
    size: 0,
  };

  for (const line of file.split("\n")) {
    if (line.startsWith("$ cd ")) {
      if (line.endsWith(" ..")) {
        if (root.parent) {
          root = root.parent;
        } else {
          // Prevent linking errors
          throw new Error("Parent not found...");
        }
      } else {
        root = root.folders.find(({ name }) => name === line.slice(5)) || root;
      }
    } else if (line.startsWith("dir ")) {
      root.folders.push({
        name: line.slice(4),
        parent: root,
        folders: [],
        files: [],
        size: 0,
      });
    } else if (line /* For empty lines */ && line !== "$ ls") {
      const matches = /(\d+)\s(\S+)/g.exec(line);
      if (matches) {
        const [_, size, name] = matches;
        root?.files.push({
          size: +size,
          name,
        });
      }
    }
  }

  // Fix the structure
  while (root.name !== "/" && root.parent) root = root.parent;

  // Update size information
  root = updateSize(root);

  // Update space left on device after knowing root size
  spaceLeft = DISK_SIZE - root.size;

  // Export data for visualization; if needed
  await Deno.writeTextFile("trie.json", JSON.stringify(root, _replacer));

  console.log({
    part_01: part01Solution(root, 1e5),
    part_02: part02Solution(root, root.size),
  });
};

const _replacer = (() => {
  const visited = new WeakSet<Directory>();
  return (_key: string, value: Directory) => {
    if (value && typeof value === "object") {
      if (visited.has(value)) return;
      visited.add(value);
    }
    return value;
  };
})();

function part01Solution(node: Directory, size: number): number {
  const limitSize = (value: number) => (value <= size ? value : 0);
  return node.folders.reduce((p, c) => {
    return p + part01Solution(c, size) + limitSize(c.size);
  }, 0);
}

function part02Solution(node: Directory, lastSize: number): number {
  const getNextSize = (value: number, acc: number) => {
    return value <= lastSize && value + spaceLeft >= MIN_SPACE ? value : acc;
  };
  return node.folders.reduce((p, c) => {
    return part02Solution(c, getNextSize(c.size, p));
  }, lastSize);
}

function updateSize(node: Directory) {
  const getSize = (data: Array<{ size: number }>) => {
    return data.reduce((p, c) => p + c.size, 0);
  };
  node.size += getSize(node.folders.map((f) => updateSize(f)));
  node.size += getSize(node.files);
  return node;
}

if (import.meta.main) {
  day07();
}
