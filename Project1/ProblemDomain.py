import abc
import ast
import math
from LinReg import LinReg
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import configparser


"""
This file contains problem domains defined in terms of the general class ProblemDomain.
Problem domains provide a fitness function given a bitstring and a function to plot of the problem (if applicable)
"""


class ProblemDomain(abc.ABC):
    @abc.abstractmethod
    def fitness_function(self, bit_string):
        raise NotImplementedError

    @abc.abstractmethod
    def plot_problem_function(self, plot_object: plt.Axes):
        raise NotImplementedError

    def scale_into_domain(self, bit_string_arr):
        raise NotImplementedError


class SineConstraint(ProblemDomain):

    def __init__(self):
        config = configparser.ConfigParser()
        config.read("config.ini")
        self.limits = ast.literal_eval(config["SINE"]["LIMITS"])
        self.x_sin = np.arange(self.limits[0], self.limits[1], 0.1)
        self.y_sin = np.sin(self.x_sin)

    def fitness_function(self, bit_string):
        scaling_factor = 2 ** (7 - len(bit_string))
        fitness = math.sin(int(bit_string, 2) * scaling_factor)
        return fitness

    def plot_problem_function(self, plot_object: plt.Axes):
        plot_object.set_title("Sine function with scaling")
        plot_object.set_xlabel("x")
        plot_object.set_ylabel("sin(x)")
        plot_object.set_xlim(self.limits[0] - 1, self.limits[1] + 1)
        plot_object.set_ylim(-1.1, 1.1)
        plot_object.set_yticks(np.arange(-1, 1.1, 0.25))
        plot_object.plot(self.x_sin, self.y_sin)

    def scale_into_domain(self, bit_string_arr):
        bs_len = len(bit_string_arr[0])
        # TODO fix scaling for arbitrary limits
        return [int(k, 2) * (2**(7 - bs_len)) for k in bit_string_arr]


class SineIndirectConstraint(ProblemDomain):

    def __init__(self):
        config = configparser.ConfigParser()
        config.read("config.ini")
        self.limits = ast.literal_eval(config["SINE_INDIRECT"]["LIMITS"])
        self.x_sin = np.arange(self.limits[0], self.limits[1], 0.1)
        self.y_sin = np.sin(self.x_sin)

    def fitness_function(self, bit_string):
        limits_delta = max(self.limits) - min(self.limits)
        limits_mid = sum(self.limits) / 2
        fitness = math.sin(int(bit_string, 2))
        # Penalty is the distance to the limit range
        penalty = 0 if self.limits[0] <= int(bit_string, 2) <= self.limits[1] else -(abs(int(bit_string, 2) - limits_mid) - limits_delta / 2)
        return fitness + penalty

    def plot_problem_function(self, plot_object: plt.Axes):
        plot_object.set_title("Sine function with indirect constraint handling")
        plot_object.set_xlabel("x")
        plot_object.set_ylabel("sin(x)")
        plot_object.set_xlim(self.limits[0] - 1, self.limits[1] + 1)
        plot_object.set_ylim(-1.2, 1.2)
        plot_object.plot(self.x_sin, self.y_sin)
        plot_object.legend(["sin(x)", "x"])

    def scale_into_domain(self, bit_string_arr):
        return [int(k, 2) for k in bit_string_arr]


class LinearRegression(ProblemDomain):

    def __init__(self):
        self.lr = LinReg()
        ds = pd.read_csv('dataset.csv', header=None)
        self.x = ds.iloc[:, :-1].values
        self.y = ds.iloc[:, -1].values

    def fitness_function(self, bit_string):
        filtered_x = self.lr.get_columns(self.x, bit_string)
        return -self.lr.get_fitness(filtered_x, self.y)

    def plot_problem_function(self, plot_object):
        plot_object.set_title("Genetic algorithm on dataset")
        plot_object.set_xlabel("x")
        plot_object.set_ylabel("-RMSE")

    def scale_into_domain(self, bit_string_arr):
        return [int(k, 2) for k in bit_string_arr]


