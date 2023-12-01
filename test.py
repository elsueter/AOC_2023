def test():
    for i in range(10):
        for j in range(10):
            break
        else:
            print(i);
            continue
        print("break")
        break

test()
