import numpy as np
import pandas as pd
from sklearn.linear_model import LinearRegression
from sklearn.model_selection import train_test_split
from sklearn.metrics import mean_squared_error
from sklearn.neighbors import KNeighborsClassifier
from sklearn.preprocessing import PolynomialFeatures
from math import sqrt


class LinReg:
    def __init__(self):
        pass

    def train(self, data, y):
        model = LinearRegression().fit(data, y)
        return model

    def get_fitness(self, x, y, random_state=42):
        if random_state == 0:
            x_train, x_test, y_train, y_test = train_test_split(x, y, test_size=0.2)
        else:
            x_train, x_test, y_train, y_test = train_test_split(x, y, test_size=0.2, random_state=random_state)
        model = self.train(x_train, y_train)
        predictions = model.predict(x_test)
        error = sqrt(mean_squared_error(predictions, y_test))

        return error

    def get_columns(self, x, bitstring):
        # Function to filter data based on a bitstring
        indexes = []
        for i, s in enumerate(bitstring):
            if s == '0':
                indexes.append(i)
        arr = np.asarray(x)
        arr = np.delete(arr, indexes, axis=1)
        return arr


l = LinReg()
m = pd.read_csv('dataset.csv', header=None)
X = m.iloc[:, :-1].values
Y = m.iloc[:, -1].values
bs = "10011001010011111100111010010111101101111010000100000100001011111111100011111001001110101110100101011"
bs_very_good = "10011111011001111010111111001100001010100100010011001001110000100001011000011100110001101011000010110"
g = l.get_columns(X, bs_very_good)

print(f"Linear Regression alone: {l.get_fitness(X, Y)}")
print(f"Best GA result: {l.get_fitness(g, Y)}")




