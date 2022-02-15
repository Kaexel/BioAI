import ProblemDomain
from GeneticAlgorithm import GeneticAlgorithm, SurvivorSelection, ParentSelection
import matplotlib.pyplot as plt
import numpy as np
import configparser
import random

""" 
----------------------------------------------------------------------------------
To run genetic algo, set relevant variables in config file and run main.
Every run does one with all config settings, then one with same settings, but survivor
selection set to crowding. Then plots them against each other.
----------------------------------------------------------------------------------
"""

""" Setting vars from config file """
available_problem_domains = {0: ProblemDomain.SineConstraint, 1: ProblemDomain.SineIndirectConstraint, 2: ProblemDomain.LinearRegression}
available_parent_selections = {0: ParentSelection.ROULETTE, 1: ParentSelection.TOURNAMENT}
available_survivor_selections = {0: SurvivorSelection.ELITISM, 1: SurvivorSelection.AGE, 2: SurvivorSelection.CROWDING}

config = configparser.ConfigParser()
config.read('config.ini')
random.seed(config["PRIMARY"].getint('RANDOM_SEED'))
problem_domain_type = config["PRIMARY"].getint('PROBLEM_DOMAIN')
plot_delay = config["PRIMARY"].getint('PLOT_DELAY')
assert (problem_domain_type in range(0, len(available_problem_domains)))
problem_domain = available_problem_domains[problem_domain_type]()

bs_length = config["GENETIC_ALGO"].getint('BIT_STRING_LENGTH')
max_generations = config["GENETIC_ALGO"].getint('MAX_GENERATIONS')
max_population = config["GENETIC_ALGO"].getint('MAX_POPULATION')
mutation_chance = config["GENETIC_ALGO"].getfloat('MUTATION_CHANCE')
crossover_chance = config["GENETIC_ALGO"].getfloat('CROSSOVER_CHANCE')
tournament_size = config["GENETIC_ALGO"].getint('TOURNAMENT_SIZE')
crowding_phi = config["GENETIC_ALGO"].getfloat('CROWDING_PHI')

survivor_selection_type = config["GENETIC_ALGO"].getint('SURVIVOR_SELECTION')
assert (problem_domain_type in range(0, len(available_problem_domains)))
survivor_selection = available_survivor_selections[survivor_selection_type]

parent_selection_type = config["GENETIC_ALGO"].getint('PARENT_SELECTION')
assert (problem_domain_type in range(0, len(available_problem_domains)))
parent_selection = available_parent_selections[parent_selection_type]


p = GeneticAlgorithm(bitstring_size=bs_length, max_generations=max_generations, population_size=max_population, survivor_selection=SurvivorSelection.ELITISM, parent_selection=ParentSelection.ROULETTE, problem_domain=problem_domain, mutation_chance=mutation_chance, crossover_chance=crossover_chance, tournament_size=tournament_size, phi=crowding_phi, plot_delay=plot_delay)
crowding = GeneticAlgorithm(bitstring_size=bs_length, max_generations=max_generations, population_size=max_population, survivor_selection=SurvivorSelection.CROWDING, parent_selection=ParentSelection.ROULETTE, problem_domain=problem_domain, mutation_chance=mutation_chance, crossover_chance=crossover_chance, tournament_size=tournament_size, phi=crowding_phi, plot_delay=plot_delay)


""" Plotting entropy comparison """
p.init_population()
p.generation_loop()

crowding.init_population()
crowding.generation_loop()

plt.xlabel("Generation #")
plt.ylabel("Entropy")
plt.title(f"Entropy")

y = p.entropies
y_p = crowding.entropies
x = np.arange(0, len(p.entropies), 1)
plt.plot(x, y, x, y_p)
plt.legend(["SGA entropy", "Crowding entropy"])
plt.show()
