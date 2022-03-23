import random
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum
from ProblemDomain import ProblemDomain


def hamming_bitstring(bs1, bs2):
    assert len(bs1) == len(bs2)
    return sum(c1 != c2 for c1, c2 in zip(bs1, bs2))


class SurvivorSelection(Enum):
    ELITISM = "Elitism"
    AGE = "Age"
    CROWDING = "Crowding"


class ParentSelection(Enum):
    ROULETTE = "Roulette"
    TOURNAMENT = "Tournament"


# Helper class to store bitstring fitness and parents (used for crowding)
class Individual:
    def __init__(self, bs, f, parents=(None, None)):
        self.bitstring = bs
        self.fitness = f
        self.parents = parents
        self.sjuk_variabel = 42069

    def __repr__(self):
        return f"Bitstring: {self.bitstring}\n Fitness: {self.fitness}\n"

    def __eq__(self, other):
        return self.bitstring == other.bitstring

    def __lt__(self, other):
        return self.fitness < other.fitness


class GeneticAlgorithm:
    def __init__(self, bitstring_size, max_generations, population_size, survivor_selection: SurvivorSelection, parent_selection: ParentSelection, problem_domain: ProblemDomain, mutation_chance=0.1, crossover_chance=1, tournament_size=3, phi=0, plot_delay=10):

        self.str_len = bitstring_size
        self.max_generations = max_generations
        self.population_size = population_size
        self.mutation_chance = mutation_chance
        self.crossover_chance = crossover_chance
        self.tournament_size = tournament_size
        self.survivor_selection = survivor_selection
        self.parent_selection = parent_selection
        self.problem_domain = problem_domain
        self.fitness_function = problem_domain.fitness_function
        self.plot_delay = plot_delay

        self.population = []
        self.phi = phi
        self.current_gen = 0
        self.entropies = []

    """Initialization and main loop"""
    def init_population(self):
        fstring = "{:0" + str(self.str_len) + "b}"  # Fixing zero padded binary strings.
        for i in range(0, self.population_size):
            individual_bitstring = fstring.format(random.randint(0, (2 ** self.str_len) - 1))
            self.population.append(Individual(individual_bitstring, self.fitness_function(individual_bitstring)))
        self.entropies.append(self.entropy())

    def generation_loop(self):
        # Loop for num generations
        for i in range(self.max_generations):
            print(f"GENERATION: {self.current_gen}")
            new_pop = []
            for j in range(0, self.population_size, 2):
                # Parent selection
                if self.parent_selection == ParentSelection.ROULETTE:
                    parents = self.roulette_selection(2)
                    children = self.crossover_offspring(parents[0], parents[1])
                    new_pop.append(children)
                elif self.parent_selection == ParentSelection.TOURNAMENT:
                    parents = self.tournament_selection()
                    children = self.crossover_offspring(parents[0], parents[1])
                    new_pop.append(children)

            # Survivor selection
            if self.survivor_selection == SurvivorSelection.ELITISM:
                for individual in new_pop:
                    self.population.append(individual[0]); self.population.append(individual[1])
                self.population.sort()
                self.population = self.population[self.population_size:]
            elif self.survivor_selection == SurvivorSelection.AGE:
                self.population = [i for tup in new_pop for i in tup]
            elif self.survivor_selection == SurvivorSelection.CROWDING:
                crowd_pop = []
                for children in new_pop:
                    p1 = children[0].parents[0]
                    p2 = children[0].parents[1]
                    c1 = children[0]
                    c2 = children[1]
                    if hamming_bitstring(p1.bitstring, c1.bitstring) + hamming_bitstring(p2.bitstring, c2.bitstring) < hamming_bitstring(p1.bitstring, c2.bitstring) + hamming_bitstring(p2.bitstring, c1.bitstring):
                        crowd_pop.append(self.general_crowding(p1, c1))
                        crowd_pop.append(self.general_crowding(p2, c2))
                    else:
                        crowd_pop.append(self.general_crowding(p1, c2))
                        crowd_pop.append(self.general_crowding(p2, c1))
                self.population = crowd_pop

            if i % self.plot_delay == 0:
                self.plot_fitness()
            self.entropies.append(self.entropy())
            self.current_gen += 1
        print(f"Best individual of final gen:\n {max(self.population)}")
        self.plot_fitness()

    """Child generating"""
    def crossover_offspring(self, parent1, parent2):
        # Crossover chance is proportion of next generation that comes from crossover.
        # If not from crossover, propagate parent bitstrings to next gen.
        if random.random() < self.crossover_chance:
            # Choose random crossover point
            crossover_point = random.randint(1, self.str_len - 1)
            child1 = parent1.bitstring[:crossover_point] + parent2.bitstring[crossover_point:]
            child2 = parent2.bitstring[:crossover_point] + parent1.bitstring[crossover_point:]
            mutated_child1 = self.mutate(child1)
            mutated_child2 = self.mutate(child2)
            return Individual(mutated_child1, self.fitness_function(mutated_child1), (parent1, parent2)), Individual(mutated_child2, self.fitness_function(mutated_child2), (parent1, parent2))
        else:
            return parent1, parent2

    def mutate(self, bitstring):
        mutated_str = ""
        for bit in bitstring:
            if random.random() < self.mutation_chance:
                mutated_str += "1" if bit == "0" else "0"
            else:
                mutated_str += bit
        return mutated_str

    """Parent selection functions"""
    def roulette_selection(self, k):
        # Pick k individuals from population (with replacement) with probability proportional to fitness
        fitness_array = np.array([k.fitness for k in self.population])
        # Ensuring all values are positive, and eliminating divide by zero errors
        fitness_array = fitness_array + abs(np.min(fitness_array)) + 1
        # Normalizing array (probabilities)
        fitness_array = fitness_array / np.sum(fitness_array)
        return random.choices(self.population,  weights=fitness_array, k=k)

    def tournament_selection(self):
        # Get parent pair from two tournaments. k, or tournament size defines number of participants in tournament
        # Picked without replacement
        t = random.sample(self.population, 2 * self.tournament_size)
        winner1 = max(t[:self.tournament_size], key=lambda i: i.fitness)
        winner2 = max(t[self.tournament_size:], key=lambda i: i.fitness)
        return [winner1, winner2]

    """ Crowding functions """
    def general_crowding(self, b1, b2):
        if b1.fitness > b2.fitness:
            p1 = b1.fitness / (b1.fitness + self.phi * b2.fitness)
        elif b1.fitness < b2.fitness:
            p1 = (self.phi*b1.fitness) / (self.phi*b1.fitness + b2.fitness)
        else:
            p1 = 0.5
        if random.random() < p1:
            return b1
        else:
            return b2

    """Assorted helper functions"""
    def entropy(self):
        p = [0] * self.str_len
        for s in self.population:
            p = [x + int(c) for x, c in zip(p, s.bitstring)]
        p = np.array(p)
        p = p / sum(p)
        p = p[p != 0]
        return -np.sum(p*np.log2(p))

    def plot_fitness(self):
        figure = plt.figure()
        ax = figure.add_subplot(1, 1, 1)
        figure.suptitle(f"Fitness of generation #{self.current_gen}\n "
                        f"Parent selection: {self.parent_selection.value}, Survivor selection: {self.survivor_selection.value}")
        self.problem_domain.plot_problem_function(ax)
        x_p = self.problem_domain.scale_into_domain([k.bitstring for k in self.population])
        y_p = [k.fitness for k in self.population]
        ax.plot(x_p, y_p, 'ro')
        figure.tight_layout()
        figure.show()
