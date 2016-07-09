import random

seed = "SEED"

def get_seed(pid, tid):
	return "%s%s%s" % (seed, pid, tid)

def get_random(pid, tid):
	random.seed(get_seed(pid, tid))
	return random
