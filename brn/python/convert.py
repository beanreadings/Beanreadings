import torch
from torch import nn

print("Running BRNN ineference script version 1.0")
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


model_name = input("Model name: ")

state = torch.load(f"models/beanreadings_neural_network_{model_name}.pth")

model = BeanreadingsNeuralNet()

model.load_state_dict(state)

torch.onnx.export(model, torch.randn(1, 16),
                  f"models/beanreadings_neural_network_{model_name}.onnx", "w")
