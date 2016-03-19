target = 500
name_value = []

def recursive(val, part):
	result = sum(part);
	if result == target:
		s = set(part)
		dic = {}
		for i in s:
			dic[i] = 0
		for i in part:
			dic[i] += 1
		for i in s:
			tup = [item for item in name_value if item[1] == i]
			if(dic[i] > 1):
				print(dic[i], tup[0][0]+'s,', end=' ')
			else:
				print(dic[i], tup[0][0]+',', end=' ')
		print()
		return
	elif result > target:
		return
	else:
		aux = 0 if not part else val.index(part[-1])
		for n in range(aux, len(val)):
			remaining = val[n+1:]
			partial = part[:]
			partial.append(val[n])
			recursive(val, partial)

def find_sums(val):
	recursive(val, [])

if __name__ == '__main__':
	values = []	
	with open('stuff.txt', encoding = 'utf-8') as file:
		for line in file:
			name, value = line.split(None, 2)
			if int(value) == target:
				print(1, name)
			elif 0 < int(value) < target:
				name_value.append( (name, int(value)) )
				values.append(int(value))
	find_sums(values)