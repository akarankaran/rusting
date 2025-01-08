num1 = 7  
num2 = 3  

# Addition
add_result = num1 + num2

# Subtraction
sub_result = num1 - num2

# Multiplication
mul_result = num1 * num2

# Division
if num2 != 0:
    div_result = num1 / num2
else:
    div_result = "Division by zero"

# Variations
# Different integers
num3 = 15  
num4 = 4  

add_result_2 = num3 + num4
sub_result_2 = num3 - num4
mul_result_2 = num3 * num4
if num4 != 0:
    div_result_2 = num3 / num4
else:
    div_result_2 = "Division by zero"

# Using negative numbers
num5 = -5  
num6 = 2  

add_result_3 = num5 + num6
sub_result_3 = num5 - num6
mul_result_3 = num5 * num6
if num6 != 0:
    div_result_3 = num5 / num6
else:
    div_result_3 = "Division by zero"

# Using zero
num7 = 0  

add_result_4 = num7 + num2
sub_result_4 = num7 - num2
mul_result_4 = num7 * num2
if num2 != 0:
    div_result_4 = num7 / num2
else:
    div_result_4 = "Division by zero"

# Large numbers
num8 = 100000  
num9 = 50000  

add_result_5 = num8 + num9
sub_result_5 = num8 - num9
mul_result_5 = num8 * num9
if num9 != 0:
    div_result_5 = num8 / num9
else:
    div_result_5 = "Division by zero"

# Small fractions as integers (effectively using int division)
num10 = 1  
num11 = 3  

add_result_6 = num10 + num11
sub_result_6 = num10 - num11
mul_result_6 = num10 * num11
if num11 != 0:
    div_result_6 = num10 / num11  # This will give a float result
else:
    div_result_6 = "Division by zero"