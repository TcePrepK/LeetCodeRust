import os
import re

import requests


def get_library_code(data):
    return f"""\
pub struct Solution {{}}

/// {data["desc"].replace("\n", "\n/// ")}

{data["snippet"]}
"""


def get_main_code(data):
    id = data["id"]
    examples = data["examples"].split("\n")
    function_name = data["functionName"]

    main_code = f"""\
use question_{id}::Solution;

fn main() {{\n"""

    for i in range(len(examples)):
        example = examples[i]
        main_code += f"    println!(\"Example {i + 1} -> {{}}\", Solution::{function_name}({example}.into()));\n"
    main_code += "}"

    return main_code


def get_cargo_toml(data):
    return f"""\
[package]
name = "question-{data['id']}"
version = "0.1.0"
edition = "2021"
"""


### AI Generated Code ###
def get_question_info(slug: str):
    url = "https://leetcode.com/graphql";
    query = """
    query getQuestionDetail($titleSlug: String!) {
      question(titleSlug: $titleSlug) {
        questionId
        title
        content
        difficulty
        likes
        dislikes
        exampleTestcases
        codeSnippets {
          lang
          langSlug
          code
        }
        sampleTestCase
      }
    }
    """
    variables = {
        "titleSlug": slug
    }
    headers = {
        "Content-Type": "application/json",
        "Referer": f"https://leetcode.com/problems/{slug}/",
        "User-Agent": "Mozilla/5.0"
    }
    response = requests.post(url, json={"query": query, "variables": variables}, headers=headers)

    if response.status_code == 200:
        data = response.json()
        question = data["data"]["question"]

        # Extract Rust snippet
        rust_snippet = ""
        for snippet in question["codeSnippets"]:
            if snippet["lang"] == "Rust":
                rust_snippet = snippet["code"]
                break
        return {
            "title": question["title"],
            "id": question["questionId"],
            "desc": question["content"],
            "difficulty": question["difficulty"],
            "examples": question["exampleTestcases"],
            "snippet": rust_snippet
        }
    else:
        print(f"Query failed. Status code: {response.status_code}")
        print(response.text)
        return None


def handle_snippet_deconstruction(data):
    snippet = data["snippet"]
    reg = r"(\w+)\s*\(.+\:\s*(.+)\)"
    matches = re.findall(reg, snippet)
    data["functionName"] = matches[0][0]
    data["functionType"] = matches[0][1]


def create_rust_project(data):
    # Get the question number and find the path
    id = data["id"]
    project_name = f"question_{id}"
    root_path = os.path.join(os.getcwd(), project_name)

    # Check if the project already exists
    if os.path.exists(root_path):
        print(f"Project {project_name} already exists!")
        return

    # Set up the project by creating the folder
    os.mkdir(root_path)
    os.mkdir(os.path.join(root_path, "src"))

    # Create the Cargo.toml file
    with open(os.path.join(root_path, "Cargo.toml"), "w") as f:
        f.write(get_cargo_toml(data))

    # Create the library file
    with open(os.path.join(root_path, "src", "lib.rs"), "w") as f:
        f.write(get_library_code(data))

    # Generate the main file
    with open(os.path.join(root_path, "src", "main.rs"), "w") as f:
        f.write(get_main_code(data))


### Adds a new row to the README statistics table.
### Table gets filled up manually.
def update_readme(data):
    with open(os.path.join(os.getcwd(), "README.md"), "r") as f:
        readme = f.readlines()

    id = data["id"]
    new_row = f'| [Question {id}](./question_{id}/src/lib.rs) | ?ms | ?mb |\n'

    insertion_index = len(readme) - 4
    readme.insert(insertion_index, new_row)

    # Write the updated README
    with open(os.path.join(os.getcwd(), "README.md"), "w") as f:
        f.writelines(readme)


if __name__ == "__main__":
    slug = input("Enter the question slug: ")
    data = get_question_info(slug)
    handle_snippet_deconstruction(data)
    create_rust_project(data)
    update_readme(data)
