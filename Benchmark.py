import random, subprocess, datetime
				#V    #edges
combinations = {50  :[50,100,200],
				100 :[100,250,500], 
				250 :[500,750,1000],
				500 :[500,1000,1500],
				1000:[1000,2000,3000],
				2500:[2500,5000]}
allowed = range(-5,94) #edge weight is uniformly distributed over this range
random.seed(1836)
itera = 3 #number of iteration per vertex-edge combination

for n in combinations:  
	for k in combinations[n]:
		negatives = []
		timef = []
		timet = []
		for i in range(itera):
			negative = 0
			edges = {i:{} for i in range(1,n+1)}
			for i in range(k):
				v,u = random.sample(range(1,n+1), 2)
				edges[v][u] = random.choice(allowed)
				negative += edges[v][u] < 0

			with open('input.txt', 'w') as file:
				file.write("V={}, E={}\n".format(n,k))

				for v in edges:
					string = "{} ".format(v)
					for u in edges[v]:
						string += "{} {} ".format(u, edges[v][u])
					string += "\n"
					file.write(string)
					
			a = subprocess.check_output('C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe .\\johnson.exe', 
				shell=True)
			b = subprocess.check_output('C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe Measure-Command{.\\johnson.exe}', 
				shell=True)
			negatives.append(negative)
			timef.append(int(a))
			timet.append(int(float(b.split()[-1])))

		print("|{}|{}|".format(n, k), end='')
		print(*negatives, sep = ',', end = '|')
		print(*timef, sep = ',', end = '|')
		print(*timet, sep = ',', end = '|\n')
