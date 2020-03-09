import Algorithmia
import torch as th
from apex import amp

"""
This package set comes preinstalled with Nvidia Apex 0.1

Example Input:
{
    "vector": [
    	[0, 1, 2, 3],
    	[4, 5, 6, 7],
    	[8, 9, 10, 11],
    	[12, 13, 14, 15]
    ]
}
"""
D_in, D_out = 4, 4

model = th.nn.Linear(D_in, D_out).cuda()
optimizer = th.optim.SGD(model.parameters(), lr=1e-3)

model, optimizer = amp.initialize(model, optimizer, opt_level="O1")

def convert(vector):
	th_tensor = th.tensor(vector).float()
	gpu_tensor = th_tensor.cuda()
	return gpu_tensor

def apply(input):
	x = convert(input["vector"])
	res = model(x)
	serialized_res = x.cpu().numpy().tolist()
	return {"result": serialized_res}