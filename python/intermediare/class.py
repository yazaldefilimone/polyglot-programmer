class Vehicle:
    def __init__(self, number_wheels, velocity, seet):
        self.number_wheels = number_wheels
        self.velocity = velocity
        self.seet = seet


car = Vehicle(2, 120, 4)

print(car)
print(car.number_wheels)
