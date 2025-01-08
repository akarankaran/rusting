def compare_numbers(num1, num2):
    if num1 > num2:
        return f"{num1} is larger than {num2}"
    elif num1 < num2:
        return f"{num2} is larger than {num1}"
    else:
        return "Both numbers are equal"

# Examples
print(compare_numbers(10, 5))          # 10 is larger than 5
print(compare_numbers(5, 10))          # 10 is larger than 5
print(compare_numbers(-3, -7))         # -3 is larger than -7
print(compare_numbers(0, 0))           # Both numbers are equal
print(compare_numbers(2.5, 2.3))       # 2.5 is larger than 2.3
print(compare_numbers(42, 42))         # Both numbers are equal
print(compare_numbers(-1, -1))         # Both numbers are equal
print(compare_numbers(100, -200))      # 100 is larger than -200
print(compare_numbers(3.14, 3.14))     # Both numbers are equal
print(compare_numbers(7, 7.0))         # Both numbers are equal