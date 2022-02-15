import GeneticAlgorithm
from GeneticAlgorithm import GeneticAlgorithm, SurvivorSelection, ParentSelection
import matplotlib.pyplot as plt
import numpy as np
#sine_fitness
#p = SGA(bitstring_size=15, max_generations=70, population_size=32, survivor_selection=SurvivorSelection.ELITISM, parent_selection=ParentSelection.ROULETTE)
#crowding = SGA(bitstring_size=15, max_generations=70, population_size=32, survivor_selection=SurvivorSelection.CROWDING, parent_selection=ParentSelection.ROULETTE)


#sine_fitness_penalty
#p = SGA(bitstring_size=15, max_generations=15, population_size=32, survivor_selection=SurvivorSelection.ELITISM, parent_selection=ParentSelection.ROULETTE)
#crowding = SGA(bitstring_size=15, max_generations=70, population_size=32, survivor_selection=SurvivorSelection.CROWDING, parent_selection=ParentSelection.TOURNAMENT)

#linear_regression
p = GeneticAlgorithm(bitstring_size=101, max_generations=70, population_size=32, survivor_selection=SurvivorSelection.ELITISM, parent_selection=ParentSelection.ROULETTE)
crowding = GeneticAlgorithm(bitstring_size=101, max_generations=70, population_size=32, survivor_selection=SurvivorSelection.CROWDING, parent_selection=ParentSelection.TOURNAMENT)

p.init_population()
p.generation_loop()

crowding.init_population()
crowding.generation_loop()

plt.xlabel("Generation #")
plt.ylabel("Entropy")
plt.title(f"Entropy")

# plt.xlim([0, 15])
# plt.ylim([-2, 2])


y = p.entropies
y_p = crowding.entropies
x = np.arange(0, len(p.entropies), 1)
plt.plot(x, y, x, y_p)
# plt.plot(x, y)
plt.legend(["Crowding entropy", "SGA entropy"])
plt.show()
