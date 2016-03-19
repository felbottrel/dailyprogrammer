import operator

def get_min_max(list):
	min_pos, min_val = min(enumerate(list), key=operator.itemgetter(1))
	max_val = max(list[ (min_pos+2)::])
	return (min_val, max_val)

def stock_market(values):
	min_max = get_min_max( [float(x) for x in values.split()] )
	print(min_max[0], " ", min_max[1])

if __name__ == '__main__':
	with open('stuff.txt', encoding = 'utf-8') as file:
		stock_market(file.read())
