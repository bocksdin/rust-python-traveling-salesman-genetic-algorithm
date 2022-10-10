# 5 cities
# genome length of 20 (1 for each connection)
# value (distance)
# lowest distance containing all 5 cities wins

import random


cities = [1, 2, 3, 4, 5]
speeds = [25, 35, 45, 55, 65]
distances = [10, 20, 30, 40, 50]
cost_to_optimize = 'distance'


class Route:
  def __init__(self, origin, destination, distance, avg_speed):
    self.origin = origin
    self.destination = destination
    self.distance = distance # in miles
    self.avg_speed = avg_speed # in miles per hour
    self.travel_time = distance / avg_speed
  # END init
# END OF ROUTE CLASS


class Traveler:
  def __init__(self, routes, generation=0):
    self.routes = routes
    self.chromosome = []
    self.cost = 0
    self.generation = generation

    for _ in range(len(routes)):
      if random.random() > 0.5:
        self.chromosome.append(1)
      else:
        self.chromosome.append(0)
  # END init

  def fitness(self):
    cost, distance, time = 0, 0, 0
    origins_visited = []
    destinations_visited = []
    combinations = []

    for i in range(len(self.chromosome)):
      if self.chromosome[i] == 1:
        origins_visited.append(self.routes[i].origin)
        destinations_visited.append(self.routes[i].destination)
        combinations.append((self.routes[i].origin, self.routes[i].destination))
        cost += getattr(self.routes[i], cost_to_optimize)
        distance += self.routes[i].distance
        time += self.routes[i].travel_time

    self.distance = distance
    self.time = time

    if self.chromosome.count(1) != len(cities):
      cost = float('inf')

    if cost < float('inf'):  
      for city in cities:
        origin_count = origins_visited.count(city)
        destination_count = destinations_visited.count(city)
        if origin_count != 1 or destination_count != 1:
          cost = float('inf')
          break

    if cost < float('inf'):      
      for combination in combinations:
        try:
          combinations.index(tuple(reversed(combination)))
          cost = float('inf')
          break
        except ValueError:
          continue

    self.cost = cost
  # END fitness

  def crossover(self, other_traveler):
    cutoff = round(len(self.chromosome) / 2)

    child1 = other_traveler.chromosome[0:cutoff] + self.chromosome[cutoff:]
    child2 = self.chromosome[0:cutoff] + other_traveler.chromosome[cutoff:]

    children = [Traveler(self.routes, self.generation + 1),
                Traveler(self.routes, self.generation + 1)]
    
    children[0].chromosome = child1
    children[1].chromosome = child2
    
    return children
  # END crossover

  def mutation(self, rate=0.01):
    for i in range(len(self.chromosome)):
      if random.random() < rate:
        if self.chromosome[i] == 0:
          self.chromosome[i] = 1
        else:
          self.chromosome[i] = 0
  # END mutation
# END OF TRAVELER CLASS


class GeneticAlgorithm:
  def __init__(self):
    self.population_size = 0
    self.population = []
    self.generation = 0
    self.best_solution = None
  # END init

  def initialize_population(self, population_size, routes):
    self.population_size = population_size
    self.routes = routes

    for _ in range(self.population_size):
      self.population.append(Traveler(self.routes))

    self.calculate_fitness()
    self.order_population()

    self.best_solution = self.population[0]
  # END initialize_population

  def calculate_fitness(self):
    for traveler in self.population:
      traveler.fitness()
  # END calculate_fitness

  def order_population(self):
    self.population = sorted(self.population, key=lambda traveler: traveler.cost)
  # END order_population

  def best_traveler(self, traveler):
    if traveler.cost < self.best_solution.cost:
      self.best_solution = traveler
  # END best_traveler

  def sum_costs(self):
    sum = 0

    for traveler in self.population:
      sum += traveler.cost

    return sum
  # END sum_costs

  def select_parent_cutoff(self, sum_cost):
    parent = -1
    random_value = random.random() * sum_cost

    sum, i = 0, 0
    while i < len(self.population) and sum < random_value:
      sum += self.population[i].cost
      parent += 1
      i += 1

    return parent
  # END select_parent_cutoff

  def visualize_generation(self):
    best = self.best_solution
    print('Generation: ', self.generation,
          '- Total Distance: ', f"{best.distance} mi", 
          '- Total Time: ', f"{best.time} hrs",
          '- Chromosome: ', best.chromosome,
          '- Cost: ', best.cost)
  # END visualize_generation

  def solve(self, mutation_probability, number_of_generations, population_size, routes):
    self.initialize_population(population_size, routes)

    for _ in range(number_of_generations):
      sum = self.sum_costs()

      new_population = []
      for _ in range(0, self.population_size, 2):
        parents = [self.select_parent_cutoff(sum),
                   self.select_parent_cutoff(sum)]
        children = self.population[parents[0]].crossover(self.population[parents[1]])

        children[0].mutation(mutation_probability)
        children[1].mutation(mutation_probability)

        new_population.append(children[0])
        new_population.append(children[1])

      self.population = list(new_population)
      self.calculate_fitness()
      self.order_population()

      if self.population[0].cost < self.best_solution.cost:
        self.best_solution = self.population[0]

      self.generation += 1
      self.visualize_generation()
            
    print('\n**** Best Solution ****',
          '\nGeneration: ', self.best_solution.generation,
          '\nTotal Distance: ', self.best_solution.distance,
          '\nTotal Time: ', self.best_solution.time,
          '\nChromosome: ', self.best_solution.chromosome)

    return self.best_solution.chromosome
  # END solve
# END OF GENETIC ALGORITHM CLASS


routes = []

for i in range(len(cities)):
  for j in range(len(cities)):
    if j == i:
      continue

    speed = random.randint(0, len(speeds) - 1)
    distance = random.randint(0, len(distances) - 1)
    routes.append(Route(cities[i], cities[j], distances[distance], speeds[speed]))

# routes.append(Route(1, 2, 10, 60)) #^
# routes.append(Route(2, 3, 10, 60)) #^
# routes.append(Route(3, 4, 10, 40)) #^
# routes.append(Route(4, 5, 10, 40)) #^
# routes.append(Route(5, 1, 10, 40)) #^
# routes.append(Route(1, 5, 10, 60)) #$
# routes.append(Route(5, 4, 10, 60)) #$
# routes.append(Route(4, 3, 10, 40)) #$
# routes.append(Route(3, 2, 10, 40)) #$
# routes.append(Route(2, 1, 10, 40)) #$
# routes.append(Route(1, 3, 15, 40)) 
# routes.append(Route(3, 1, 15, 40)) 
# routes.append(Route(1, 4, 15, 40)) 
# routes.append(Route(4, 1, 15, 40)) 
# routes.append(Route(2, 4, 15, 40)) 
# routes.append(Route(4, 2, 15, 40)) 
# routes.append(Route(2, 5, 15, 40)) 
# routes.append(Route(5, 2, 15, 40)) 
# routes.append(Route(3, 5, 15, 40)) 
# routes.append(Route(5, 3, 15, 40)) 

mutation_probability = 0.05
number_of_generations = 1000
population_size = 2000
ga = GeneticAlgorithm()

result = ga.solve(mutation_probability,
                  number_of_generations,
                  population_size,
                  routes)

print('\n**** Route Taken ****')
for i in range(len(result)):
  if result[i] == 1:
    print(routes[i].origin, '->', routes[i].destination, ':', f'{routes[i].distance}mi', f'{routes[i].avg_speed}mph', f'{routes[i].travel_time}hrs')
