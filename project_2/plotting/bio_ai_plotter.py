import matplotlib.pyplot as plt
import json
import os
import glob
import ast
dirname = os.path.dirname(__file__)
solution = glob.glob("solution.txt")
print(solution)

if solution:
    with open(solution[0]) as solution:
        train_file = solution.readline().strip()
        solution_list = ast.literal_eval(solution.readline())
    print(train_file + ".json")
    data = glob.glob("train\\" + train_file + ".json")
    print(data)
    with open(data[0]) as f:
        json_file = json.load(f)
        print(json_file["depot"])
        depot = (json_file["depot"]["x_coord"], json_file["depot"]["y_coord"])
        patients = json_file["patients"]
        coords = [(patient["x_coord"], patient["y_coord"]) for patient in json_file["patients"].values()]

    coords_separated = list(zip(*coords))
    plt.scatter(coords_separated[0], coords_separated[1])
    plt.plot(depot[0], depot[1], marker="o", markersize=7, markeredgecolor="green", markerfacecolor="green")
    plt.title(data)
    for list_s in solution_list:
        x = [depot[0]]
        y = [depot[1]]
        for patient in list_s:
            x.append(patients[str(patient)]["x_coord"])
            y.append(patients[str(patient)]["y_coord"])
        x.append(depot[0])
        y.append(depot[1])

        plt.plot(x, y)

    plt.show()
