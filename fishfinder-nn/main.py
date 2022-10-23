import fishgen

fish = fishgen.random_fish(512, 512, 0.1, 1.0)
box = fishgen.random_box(512, 512, 0.1, 1.0)
squiggle = fishgen.random_squiggle(512, 512, 0.1, 1.0)
decoy = fishgen.random_decoy_fish(512, 512, 0.1, 1.0)

print(fish.image.shape)
print(box.image.shape)
print(squiggle.image.shape)
print(decoy.image.shape)
