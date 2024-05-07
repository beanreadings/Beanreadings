import torchinfo
import torch
from torch import nn
import pandas as pd
import time

print("Running BRNN training script version 1.0")
print("Bean Readings Neural Network")
print()
print("PyTorch version: ", torch.__version__)
print("CUDA available: ", torch.cuda.is_available())

# here is where we define the model


class BeanreadingsNeuralNet(nn.Module):
    def __init__(self):
        super(BeanreadingsNeuralNet, self).__init__()
        # beanreadings neural network has one input layer,
        # with 16 input features, three hidden layers with
        # 48 neurons each, and one output layer with 1 output

        self.input_layer = nn.Linear(16, 48)
        self.hidden_layer1 = nn.Linear(48, 48)
        self.hidden_layer2 = nn.Linear(48, 48)
        self.hidden_layer3 = nn.Linear(48, 48)
        self.hidden_layer4 = nn.Linear(48, 48)
        self.hidden_layer5 = nn.Linear(48, 48)
        self.hidden_layer6 = nn.Linear(48, 48)
        self.hidden_layer7 = nn.Linear(48, 48)
        self.output_layer = nn.Linear(48, 1)

    def forward(self, x):
        x = torch.relu(self.input_layer(x))
        x = torch.relu(self.hidden_layer1(x))
        x = torch.relu(self.hidden_layer2(x))
        x = torch.relu(self.hidden_layer3(x))
        x = torch.relu(self.hidden_layer4(x))
        x = torch.relu(self.hidden_layer5(x))
        x = torch.relu(self.hidden_layer6(x))
        x = torch.relu(self.hidden_layer7(x))
        x = self.output_layer(x)
        return x


# all the training data is available as data.csv here
# we parse the data using pandas and split it into
# input features X and output y

data = pd.read_csv("data.csv")

print()
print(data.head())
print()

# calcium , sugar, solid_fats, fibre, salt, smoking rate, alcohol rate,
# binge drinking rate, vaping rate, exercise rate, sleep rate, hydration
# rate, anti vacc rate, drug rate, years, starting population,
# intended population

X = data.iloc[:, 0:16].values
y = data.iloc[:, 16].values

# we convert the data into PyTorch tensors

X = torch.tensor(X, dtype=torch.float32)

y = torch.tensor(y, dtype=torch.float32)

# we want beanreadings neural network to use the first 16 rows, and then
# predict the last one, the "intended population"

epochs = 1000000

learning_rate = 2e-3

model = BeanreadingsNeuralNet()

torchinfo.summary(model)

print()

criterion = nn.MSELoss()

optimizer = torch.optim.Adam(model.parameters(), lr=learning_rate)

for epoch in range(epochs):
    optimizer.zero_grad()
    y_pred = model(X).flatten()
    loss = criterion(y_pred, y)
    loss.backward()
    optimizer.step()
    print(f"Epoch {epoch} loss: {loss.item()}")
    if epoch % 1000 == 0:
        timestamp = time.time()

        print("Saving checkpoint")

        torch.save(model.state_dict(),
                   f"checkpoints/bnn_{round(timestamp)}_{str(int(loss.item()))}.pth")

# we save the weights to a file, i will run them in rust later with
# my cool rust skills

torch.save(model.state_dict(), "beanreadings_neural_network_1000.pth")
