# Unique snowflakes

Given a collection of snowflakes, determine whether any of the snowflakes are identical

## Pretext

A snowflake is represented by  6 integer

Identicial snowflakes means the integers are identicial 

e.g. 

1,2,3,4,5,6 = 4,5,6,1,2,3
            = 6,5,4,3,2,1


## Input

```
n <- 1..100,000

x_0,1, x_0,2, x_0,3, x_0,4, x_0,5, x_0,6
.
.
x_(n-1), 1, x_(n-1), 2, x_(n-1), 3, x_(n-1), 4, x_(n-1), 5, x_(n-1), 6

```


## Output
```
If not identifical: "no two snowflakes are alike"

If there are two identifical snowflakes, "Twin snowflakes found"
```
