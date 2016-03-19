def find_divisors(n):
	for i in range(n):
		if n % i == 0:
			yield i

def main_function(n):
	sigma = n
	n2 = 2 * n
	for i in find_divisors(n):
		sigma += i
	if sigma > n2:
		print("{0} abundant by {1}".format(n, sigma-n2))
	elif sigma < n2:
		print("{0} deficient".format(n))
	else:
		print("{0} perfect".format(n))

if __name__ == '__main__':
	values = [18, 21, 9, 111, 112, 220, 69, 134, 85, 6]
	for i in values:
		main_function(i)