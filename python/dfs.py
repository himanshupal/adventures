from collections import deque


"""
https://brilliant.org/wiki/depth-first-search-dfs
https://brilliant.org/wiki/breadth-first-search-bfs
"""


def bfs(graph: dict[int, set[int]], root: int = 1):
    """
    Breadth-first search algorithm
    """
    visited, queue = set([root]), deque([root])
    # print(f"Visited: {root}")
    while queue:
        vertex = queue.popleft()
        for next in graph[vertex].difference(visited):
            # print(f"Visited: {next}")
            queue.append(next)
            visited.add(next)
    return visited


def dfs(graph: dict[int, set[int]], root: int = 1):
    """
    Depth-first search algorithm
    """
    visited, stack = set(), [root]
    while stack:
        vertex = stack.pop()
        if vertex not in visited:
            visited.add(vertex)
            # print(f"Visited: {vertex}")
            stack.extend(graph[vertex].difference(visited))
    return visited


def dfs_r(graph: dict[int, set[int]], root: int = 1, visited: set[int] = set()):
    """
    Depth-first search recursive algorithm
    """
    visited.add(root)
    # print(f"Visited: {root}")
    for next in graph[root].difference(visited):
        dfs_r(graph, next, visited)
    return visited


dfs_graph = {
    1: {2, 7, 8},
    2: {1, 3, 6},
    3: {2, 4, 5},
    4: {3},
    5: {3},
    6: {2},
    7: {1},
    8: {1, 9, 12},
    9: {8, 10, 11},
    10: {9},
    11: {9},
    12: {8}
}

bfs_graph = {
    1: {2, 3, 4},
    2: {1, 5, 6},
    3: {1},
    4: {1, 7, 8},
    5: {2, 9, 10},
    6: {2},
    7: {4, 11, 12},
    8: {4},
    9: {5},
    10: {5},
    11: {7},
    12: {7}
}

print(bfs(bfs_graph))
print(dfs(dfs_graph))
print(dfs_r(dfs_graph))
