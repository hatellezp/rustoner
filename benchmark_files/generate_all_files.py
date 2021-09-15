import os
from typing import List, Tuple
import random
import graphviz

def read_symbosl():
    role_symbols = []
    concept_symbols = []

    with open("new_onto/symbols.txt") as f:
        for line in f.readlines():
            line = line.strip()
            if line[:2] == "//" or len(line) == 0:
                continue

            if line == "BEGINSYMBOL" or line == "ENDSYMBOL":
                continue

            type, name = line.split(":")
            type = type.strip()
            name = name.strip()

            if type == "role":
                role_symbols.append(name)
            elif type == "concept":
                concept_symbols.append(name)
            else:
                raise Exception("unknown type of symbol")

    return role_symbols, concept_symbols


class Node:
    def __init__(self, t: str, n: str, ir: bool = False):
        self.type = t
        self.name = n
        self.is_root = ir

        self.children = []

        self.depth = 0

    def add_child(self, child: 'Node'):
        if self.type == child.type:
            self.children.append(child)
            child.depth = self.depth + 1
        else:
            exception = f"child [{child.name}, {child.type}] is not of the same type as parent [{self.name}, {self.type}]!!!"
            raise Exception(exception)

    def __str__(self):
        children_str = "\n".join(list(str(child) for child in self.children))
        return f"[{self.name}, {self.type}]\n  {children_str}"


def generate_role_tree(concept_list: List[str],
                          size: int,
                          branching_factor: int,
                          depth: int,
                          edge_possibility: float) -> List[Node]:
    counter = 0
    # root = None
    # an element of this list is as follows:
    # (node, current branching of this node, current depth of this node)
    existent_nodes: List[Node] = []

    while counter < size:
        new_name: str = random.choice(concept_list)
        new_node = Node("role", new_name)

        if len(existent_nodes) == 0:
            existent_nodes.append(new_node)
            continue

        if random.random() > (1. - edge_possibility):
            done = False
            while not done:
                parent = random.choice(existent_nodes)

                if len(parent.children) < branching_factor and parent.depth < depth and not(new_node in parent.children):
                    parent.add_child(new_node)
                    done = True

        existent_nodes.append(new_node)
        counter += 1

    return existent_nodes


def generate_concept_tree(concept_list: List[str],
                          size: int,
                          branching_factor: int,
                          depth: int,
                          edge_possibility: float,
                          exists_possibility: float,
                          conflict_possibility: float,
                          roles_names: List[str],
                          role_edges: List[Tuple[str, str]]) -> List[Node]:
    counter = 0
    # root = None
    # an element of this list is as follows:
    # (node, current branching of this node, current depth of this node)
    existent_nodes: List[Node] = []

    while counter < size:
        new_name: str = random.choice(concept_list)

        if random.random() > (1 - exists_possibility):
            role_name = random.choice(roles_names)
            new_name = "EXISTS " + role_name

        if random.random() > (1 - conflict_possibility) and len(existent_nodes) != 0:
            new_name = "NOT " + new_name

        new_node = Node("concept", new_name)

        if len(existent_nodes) == 0:
            existent_nodes.append(new_node)
            continue

        if random.random() > (1. - edge_possibility):

            done = False
            while not done:
                parent = random.choice(existent_nodes)

                if (len(parent.children) < branching_factor) and (parent.depth < depth) and (not(new_node in parent.children)) and ("NOT" not in parent.name):
                    parent.add_child(new_node)
                    done = True

        existent_nodes.append(new_node)
        counter += 1

    return existent_nodes


def from_concept_tree_specification_to_graph(nodes):
    vertex = []
    edges = []

    for node in nodes:
        new_node = Node(node.type, node.name)

        vertex.append(new_node)

        for child in node.children:
            other_new_node = Node(child.type, child.name)

            edges.append((new_node, other_new_node))

    return vertex, edges


if __name__ == '__main__':

    role_symbols, concept_symbols = read_symbosl()


    """
        what is the plan:
            - make a little forest of roles from parameters 
            - triple the parameters and make a forest of concepts taking into account the roles
            - at the end (when current_depth == depth - 1) add the possibility of negating the concept 
              and thus introduce a conflict
    """

    """ 
        size that I'm going to use:
            tbox:
                sizes: 10, 50, 100
            abox:
                size: 10, 50, 100, 200, 400, 500, 600, 800 1000 
                interaction density: 0.1, 0.2, 0.5, 1
    """

    parameters = [
        {
            "role_size": 2,
            "role_branching_factor": 2,
            "role_depth": 3,
            "role_number_of_trees": 2,
            "role_edge_possibility": 0.5,
            "role_joining_tree_possibility": 0.4,
            "concept_size": 8,
            "concept_branching_factor": 3,
            "concept_depth": 4,
            "concept_number_of_trees": 4,
            "concept_conflict_possibility": 0.3,
            "concept_exists_possibility": 0.2,
            "concept_edge_possibility": 0.5,
            "concept_joining_tree_possibility": 0.4,
        },
    ]

    iterations = 5
    for iteration in range(iterations):
        for (index, parameter_dict) in enumerate(parameters):

            print(f"doing it {index} times")

            role_size = parameter_dict["role_size"]
            role_branching_factor = parameter_dict["role_branching_factor"]
            role_depth = parameter_dict["role_depth"]
            role_number_of_trees = parameter_dict["role_number_of_trees"]
            role_edge_possibility = parameter_dict["role_edge_possibility"]
            role_joining_tree_possibility = parameter_dict["role_joining_tree_possibility"]

            concept_size = parameter_dict["concept_size"]
            concept_branching_factor = parameter_dict["concept_branching_factor"]
            concept_depth = parameter_dict["concept_depth"]
            concept_number_of_trees = parameter_dict["concept_number_of_trees"]

            concept_conflict_possibility = parameter_dict["concept_conflict_possibility"]
            concept_exists_possibility = parameter_dict["concept_exists_possibility"]
            concept_edge_possibility = parameter_dict["concept_edge_possibility"]
            concept_joining_tree_possibility = parameter_dict["concept_joining_tree_possibility"]

            dot = graphviz.Digraph()

            keeper_of_vertex = []

            role_dot_edges = []
            role_dot_nodes = []

            # role forest
            for i in range(role_number_of_trees):
                role_forest = generate_role_tree(role_symbols, role_size // role_number_of_trees, role_branching_factor, role_depth, role_edge_possibility)

                print(len(role_forest))

                vertex, edges = from_concept_tree_specification_to_graph(role_forest)

                keeper_of_vertex.append(vertex.copy())

                for ver in vertex:
                    dot.node(ver.name, ver.name)

                    if ver.name not in role_dot_nodes:
                        role_dot_nodes.append(ver.name)

                for edge in edges:
                    a, b = edge
                    dot.edge(a.name, b.name)

                    if (a.name, b.name) not in role_dot_edges:
                        role_dot_edges.append((a.name, b.name))

            # adding branching between trees
            for i in range(role_number_of_trees - 1):
                node_i = random.choice(keeper_of_vertex[i])
                for j in range(i + 1, role_number_of_trees):
                    node_j = random.choice(keeper_of_vertex[j])

                    if random.random() > (1 - role_joining_tree_possibility):
                        dot.edge(node_i.name, node_j.name)

                        if (node_i.name, node_j.name) not in role_dot_edges:
                            role_dot_edges.append((node_i.name, node_j.name))

            keeper_of_vertex = []

            concept_dot_nodes = []
            concept_dot_edges = []

            for i in range(concept_number_of_trees):
                concept_forest = generate_concept_tree(concept_symbols, concept_size // concept_number_of_trees, concept_branching_factor, concept_depth, concept_edge_possibility, concept_exists_possibility, concept_conflict_possibility, role_dot_nodes, role_dot_edges)

                print(len(concept_forest))

                vertex, edges = from_concept_tree_specification_to_graph(concept_forest)

                keeper_of_vertex.append(vertex.copy())

                for ver in vertex:
                    dot.node(ver.name, ver.name)

                    if ver.name not in concept_dot_nodes:
                        concept_dot_nodes.append(ver.name)

                for edge in edges:
                    a, b = edge
                    dot.edge(a.name, b.name)

                    if (a.name, b.name, "NOT" in b.name) not in concept_dot_edges:
                        concept_dot_edges.append((a.name, b.name, "NOT" in b.name))

            for i in range(concept_number_of_trees - 1):
                node_i = random.choice(keeper_of_vertex[i])
                for j in range(i + 1, concept_number_of_trees):
                    node_j = random.choice(keeper_of_vertex[j])

                    if random.random() > (1 - concept_joining_tree_possibility):
                        dot.edge(node_i.name, node_j.name)

                        if (node_i.name, node_j.name, "NOT" in node_j.name) not in concept_dot_edges:
                            concept_dot_edges.append((node_i.name, node_j.name, "NOT" in node_j.name))

            dot_edges = []
            dot_nodes = []

            for element in dot.body:
                if "->" in element:
                    dot_edges.append(element)
                else:
                    dot_nodes.append(element)


            print(f"nodes: {len(dot_nodes)}, edges: {len(dot_edges)}")


            # here create the Ontology
            root_dir = "new_onto/"

            onto_dir_name = f"Onto_n{len(role_dot_nodes) + len(concept_dot_nodes)}_d{max(role_depth, concept_depth)}_c{len([x for x in concept_dot_nodes if x[2]])}_i{iteration}"

            onto_path = root_dir + onto_dir_name
            if not os.path.exists(onto_path):
                os.makedirs(onto_path)

            def clean_concept_name(name):
                if "NOT" in name:
                    name = name.split("NOT")[1].strip()

                if "EXISTS" in name:
                    name = name.split("EXISTS")[1].strip()

                return name

            with open(onto_path + "/tbox.txt", 'w+') as f:

                # write symbols
                f.write("BEGINSYMBOL\n")

                # write roles
                for role_name in role_dot_nodes:
                    f.write(f"role : {role_name}\n")

                # write concept
                for concept_name in concept_dot_nodes:
                    f.write(f"concept : {clean_concept_name(concept_name)}\n")

                f.write("ENDSYMBOL\n")

                f.write("\n")

                # write tbox
                f.write("BEGINTBOX\n")

                # write role axioms
                for axiom in role_dot_edges:
                    (a, b) = axiom
                    f.write(f"{a} < {b}\n")

                # write concept axioms

                for axiom in concept_dot_edges:
                    (a, b, _) = axiom
                    f.write(f"{a} < {b}\n")

                f.write("ENDTBOX\n\n")

            dot.render('test-output/round-table.gv', view=True)
