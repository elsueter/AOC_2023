def read_file(path):
    f = open(path, "r")
    return f

def part_1():
    
    highest = 0
    current = 0

    for line in read_file("input.txt"):
        if line != "\n":
            current += int(line)
        else:
            if current > highest:
                highest = current
            current = 0

    return highest

def part_2():
    
    highest = [0, 0, 0]
    current = 0

    for line in read_file("input.txt"):
        if line != "\n":
            current += int(line)
        else:
            for i, elf in enumerate(highest):
                if current > elf:
                    highest.insert(i, current)
                    highest.pop()
                    break
            current = 0

    return highest[0]+highest[1]+highest[2]


print(part_1())
print(part_2())

def quicksort(arr, lo, hi):
    if lo >= hi:
        return

    pivot = partition(arr, lo, hi)

    quicksort(arr, lo, pivot-1)
    quicksort(arr, pivot+1, hi)

def partition(arr, lo, hi):

    pivot = arr[hi]
    idx = lo - 1

    for i in range(lo, hi):
        if arr[i] <= pivot:
            idx = idx + 1
            tmp = arr[i]
            arr[i] = arr[idx]
            arr[idx] = tmp

    idx = idx + 1
    arr[hi] = arr[idx]
    arr[idx] = pivot

    return idx

random_arr = [2, 10, 3, 5, 1, 2, 9, 7]
print(random_arr)
quicksort(random_arr, 0, len(random_arr) - 1)
print(random_arr)
