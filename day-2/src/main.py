# from pprint import pprint


def invalid(id):
    id = str(id)
    le = len(id)
    return id[: le // 2] == id[le // 2 :]


file = "../inputs/2.txt"
with open(file) as f:
    content = f.readlines()

ranges = content[0].split(",")
answer = 0
print(content)
for id_range in ranges:
    low, high = map(int, id_range.split("-"))
    # print(low, high)
    for id in range(low, high + 1):
        if invalid(id):
            answer += id
print(answer)
