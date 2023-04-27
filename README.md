# Filler
## Project

[Filler](https://github.com/01-edu/public/tree/master/subjects/filler) is an algorithmic game which consists in filling a grid of a known size in advance with pieces of a random size and shape, provided by the game_engine. In this game, two robots fight against each other, one after the other on the Anfield.
And so I present to you my robot : Muggle

## Muggle
Muggle certainly lacks any sort of magical ability, he was not born in magical family, but he can do the things that need to be done. And he calculates the best attacking angle based on closest opponent piece and distance from the edge. Mainly putting pressure to the middle of the field. But he has some tricks up his sleeve, so don't underestimate him. As long as you are not terminator, then feel free to underestimate, because the muggle can't stand a chance against him. 

## Build and run
Project as contains only Muggle robot, the game engine, maps and robots that are provided will be added through docker.
So all it takes is building the docker image

```
docker build -t filler .
```

Then launch the docker container 

```
docker run -it filler
```
**-it** flag takes you inside of the container, so you can interact with container through terminal

Now you can start the robot battle:

```
./game_engine -f maps/map01 -p1 robots/muggle -p2 robots/bender
```

Here are the flags and what they mean:

```
  -f, -file string
       Path to map
  -p1, -player1 string
       Path to AI one
  -p2, -player2 string
       Path to AI two
  -q, -quiet
       Quiet mode
```
## Helper

 And here is a little helper, so you can just copy and paste faster:
 ```bash
 #setup
 docker build -t filler .
 docker run -it filler
```
```bash
#1 bender vs terminator
./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
```
```bash
#2 student vs wall_e
./game_engine -f maps/map01 -p1 robots/muggle -p2 robots/wall_e
```
```bash
#3 student vs wall_e
./game_engine -f maps/map00 -p1 robots/muggle -p2 robots/wall_e
```
```bash
./game_engine -f maps/map00 -p1 robots/wall_e -p2 robots/muggle
```
```bash
#4 student vs h2_d2
./game_engine -f maps/map01 -p1 robots/muggle -p2 robots/h2_d2
```
```bash
./game_engine -f maps/map01 -p1 robots/h2_d2 -p2 robots/muggle
```
```bash
#4 student vs bender
./game_engine -f maps/map02 -p1 robots/muggle -p2 robots/bender
```
```bash
./game_engine -f maps/map02 -p1 robots/bender -p2 robots/muggle

 ```