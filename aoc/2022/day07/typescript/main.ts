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

const day07_01 = async () => {
  const file = await Deno.readTextFile("../input.txt");

  let root: Directory | undefined = {
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
  while (root?.name !== "/") root = root?.parent;

  // Update size information
  root = updateSize(root);

  // Export data for visualization; if needed
  // Deno.writeTextFile("trie.json", JSON.stringify(root, _replacer));

  console.log({ part_01: part01Solution(root, 1e5) });
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

  if (node.folders.length)
    return node.folders.reduce((p, c) => {
      return p + part01Solution(c, size) + limitSize(c.size);
    }, 0);

  return 0;
}

function getSize(data: Array<{ size: number }>) {
  return data.reduce((p, c) => p + c.size, 0);
}

function updateSize(node: Directory) {
  if (node.folders.length) {
    node.size += getSize(node.folders.map((f) => updateSize(f)));
  }

  if (node.files.length) {
    node.size += getSize(node.files);
  }

  return node;
}

if (import.meta.main) {
  day07_01();
}
