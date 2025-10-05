"""
base:
Benchmark 1: cargo build --release
  Time (mean ± σ):     58.150 s ±  0.163 s    [User: 758.211 s, System: 37.637 s]
  Range (min … max):   57.936 s … 58.321 s    5 runs

thin:
Benchmark 1: cargo build --release
Time (mean ± σ):     63.999 s ±  0.105 s    [User: 879.703 s, System: 40.045 s]
Range (min … max):   63.921 s … 64.182 s    5 runs

fat:
Time (mean ± σ):     264.606 s ±  2.238 s    [User: 570.800 s, System: 31.826 s]
Range (min … max):   261.573 s … 267.297 s    5 runs

# cargo

base:
Benchmark 1: cargo build --release
  Time (mean ± σ):     89.381 s ±  0.460 s    [User: 689.874 s, System: 55.347 s]
  Range (min … max):   88.605 s … 89.696 s    5 runs

thin:
Benchmark 1: cargo build --release
  Time (mean ± σ):     91.208 s ±  0.610 s    [User: 757.353 s, System: 58.558 s]
  Range (min … max):   90.415 s … 92.112 s    5 runs

fat:
Time (mean ± σ):     212.215 s ±  2.062 s    [User: 576.259 s, System: 50.961 s]
Range (min … max):   208.662 s … 213.818 s    5 runs

# ripgrep

base:
Time (mean ± σ):      7.507 s ±  0.223 s    [User: 64.115 s, System: 4.514 s]
Range (min … max):    7.357 s …  7.882 s    5 runs

thin:
Time (mean ± σ):      9.285 s ±  0.019 s    [User: 81.101 s, System: 5.241 s]
Range (min … max):    9.262 s …  9.308 s    5 runs

fat:
Time (mean ± σ):     29.202 s ±  0.279 s    [User: 51.015 s, System: 3.652 s]
Range (min … max):   28.860 s … 29.574 s    5 runs

# triagebot

base:
Time (mean ± σ):     74.532 s ±  0.378 s    [User: 766.778 s, System: 58.719 s]
Range (min … max):   74.105 s … 75.109 s    5 runs

thin:
Time (mean ± σ):     89.505 s ±  0.299 s    [User: 1523.951 s, System: 102.429 s]
Range (min … max):   89.024 s … 89.796 s    5 runs

fat:
Time (mean ± σ):     273.275 s ±  1.694 s    [User: 929.604 s, System: 65.856 s]
Range (min … max):   271.007 s … 275.619 s    5 runs
"""

data = [
    {
        "name": "r-a",
        "base": 58.150,
        "thin": 63.999,
        "fat": 264.606,
    },
    {
        "name": "cargo",
        "base": 89.381,
        "thin": 91.208,
        "fat": 212.215,
    },
    {
        "name": "ripgrep",
        "base": 7.507,
        "thin": 9.285,
        "fat": 29.202,
    },
    {
        "name": "triagebot",
        "base": 74.532,
        "thin": 89.505,
        "fat": 273.275,
    }
]

for bench in data:
    print(f"{bench["name"]} ThinLTO: {bench["thin"] / bench["base"]}")
    print(f"{bench["name"]} Fat LTO: {bench["fat"] / bench["base"]}")

def avg_of(scenario: str) -> float:
    avg_percentage = sum([bench[scenario] / bench["base"] for bench in data]) / len(data)
    return avg_percentage

print(f"ThinLTO: {avg_of("thin")}")
print(f"Fat LTO: {avg_of("fat")}")
