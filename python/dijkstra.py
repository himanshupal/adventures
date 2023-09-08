import sys

# Providing the graph
vertices = [
    [0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0],
    [1, 1, 0, 1, 1, 0, 0],
    [1, 0, 1, 0, 0, 0, 1],
    [0, 0, 1, 0, 0, 1, 0],
    [0, 1, 0, 0, 1, 0, 1],
    [0, 0, 0, 1, 0, 1, 0]
]

edges = [
    [0, 0, 1, 2, 0, 0, 0],
    [0, 0, 2, 0, 0, 3, 0],
    [1, 2, 0, 1, 3, 0, 0],
    [2, 0, 1, 0, 0, 0, 1],
    [0, 0, 3, 0, 0, 2, 0],
    [0, 3, 0, 0, 2, 0, 1],
    [0, 0, 0, 1, 0, 1, 0]
]


def to_be_visited():
    """
    Find which vertex is to be visited next
    """
    global visited_and_distance
    v = -10
    for index in range(num_of_vertices):
        if visited_and_distance[index][0] == 0 \
            and (v < 0 or visited_and_distance[index][1] <=
                 visited_and_distance[v][1]):
            v = index
    return v


num_of_vertices = len(vertices[0])

visited_and_distance = [[0, 0]]
for i in range(num_of_vertices-1):
    visited_and_distance.append([0, sys.maxsize])

for vertex in range(num_of_vertices):
    """
    Find next vertex to be visited
    """
    to_visit = to_be_visited()
    for neighbor_index in range(num_of_vertices):

        # Updating new distances
        if vertices[to_visit][neighbor_index] == 1 and \
                visited_and_distance[neighbor_index][0] == 0:
            new_distance = visited_and_distance[to_visit][1] \
                + edges[to_visit][neighbor_index]
            if visited_and_distance[neighbor_index][1] > new_distance:
                visited_and_distance[neighbor_index][1] = new_distance

        visited_and_distance[to_visit][0] = 1

i = 0

# Printing the distance
for distance in visited_and_distance:
    print("Distance of ", chr(ord('a') + i),
          " from source vertex: ", distance[1])
    i = i + 1


# ==================================================

class Graph(object):
    def __init__(self, nodes, init_graph):
        self.nodes = nodes
        self.graph = self.construct_graph(nodes, init_graph)

    def construct_graph(self, nodes, init_graph):
        '''
        This method makes sure that the graph is symmetrical. In other words, if there's a path from node A to B with a value V, there needs to be a path from node B to node A with a value V.
        '''
        graph = {}
        for node in nodes:
            graph[node] = {}

        graph.update(init_graph)

        for node, edges in graph.items():
            for adjacent_node, value in edges.items():
                if graph[adjacent_node].get(node, False) == False:
                    graph[adjacent_node][node] = value

        return graph

    def get_nodes(self):
        "Returns the nodes of the graph."
        return self.nodes

    def get_outgoing_edges(self, node):
        "Returns the neighbors of a node."
        connections = []
        for out_node in self.nodes:
            if self.graph[node].get(out_node, False) != False:
                connections.append(out_node)
        return connections

    def value(self, node1, node2):
        "Returns the value of an edge between two nodes."
        return self.graph[node1][node2]


def dijkstra_algorithm(graph, start_node):
    unvisited_nodes = list(graph.get_nodes())

    # We'll use this dict to save the cost of visiting each node and update it as we move along the graph
    shortest_path = {}

    # We'll use this dict to save the shortest known path to a node found so far
    previous_nodes = {}

    # We'll use max_value to initialize the "infinity" value of the unvisited nodes
    max_value = sys.maxsize
    for node in unvisited_nodes:
        shortest_path[node] = max_value
    # However, we initialize the starting node's value with 0
    shortest_path[start_node] = 0

    # The algorithm executes until we visit all nodes
    while unvisited_nodes:
        # The code block below finds the node with the lowest score
        current_min_node = None
        for node in unvisited_nodes:  # Iterate over the nodes
            if current_min_node == None:
                current_min_node = node
            elif shortest_path[node] < shortest_path[current_min_node]:
                current_min_node = node

        # The code block below retrieves the current node's neighbors and updates their distances
        neighbors = graph.get_outgoing_edges(current_min_node)
        for neighbor in neighbors:
            tentative_value = shortest_path[current_min_node] + \
                graph.value(current_min_node, neighbor)
            if tentative_value < shortest_path[neighbor]:
                shortest_path[neighbor] = tentative_value
                # We also update the best path to the current node
                previous_nodes[neighbor] = current_min_node

        # After visiting its neighbors, we mark the node as "visited"
        unvisited_nodes.remove(current_min_node)

    return previous_nodes, shortest_path


def print_result(previous_nodes, shortest_path, start_node, target_node):
    path = []
    node = target_node

    while node != start_node:
        path.append(node)
        node = previous_nodes[node]

    # Add the start node manually
    path.append(start_node)

    print(
        f"We found the following best path with a value of {shortest_path[target_node]}.")
    print(" -> ".join(reversed(path)))


init_graph: dict[str, dict[str, int]] = {
    "Reykjavik": {
        "Oslo": 5,
        "London": 4,
    },
    "Oslo": {
        "Berlin": 1,
        "Moscow": 3,
    },
    "Moscow": {
        "Belgrade": 5,
        "Athens": 4,
    },
    "London": {
        # No path
    },
    "Rome": {
        "Berlin": 2,
        "Athens": 2,
    },
    "Berlin": {
        # No path
    },
    "Belgrade": {
        # No path
    },
    "Athens": {
        "Belgrade": 1,
    },
}

graph = Graph(init_graph.keys(), init_graph)
previous_nodes, shortest_path = dijkstra_algorithm(graph, "Reykjavik")
print_result(previous_nodes, shortest_path, "Reykjavik", "Belgrade")
