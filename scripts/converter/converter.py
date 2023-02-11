from PIL import Image 
import numpy as np
DEFAULT_COLOR = [255,255,255] # color which indicates no grass
MAX_CLUSTER_SIZE = 20
RAW_FILE_PATH = '../unformated/grass_placement.png'
OUTPUT_FILE_PATH = "../../assets/grass_placement.ron"
def grass_height(color):
    max_height = np.linalg.norm(DEFAULT_COLOR)
    distance = np.linalg.norm(color[:3] - DEFAULT_COLOR)
    height = distance / max_height
    if height < 0.1:
        return 0
    else:
        return height


class Box:
    def __init__(self, color, x, y):
        self.height = grass_height(color)
        self.x = x
        self.y = y
        self.w = 1
        self.h = 1
    def can_add_color(self, color, x, y):
        if self.w == MAX_CLUSTER_SIZE:
            return False
        if self.x + self.w != x:
            return(False)
        if self.y  != y:
            return False
        return grass_height(color) == self.height
    def try_add_color(self, color, x, y):
        if not self.can_add_color(color,x,y):
            return False
        self.w += 1
        return True
    def __str__(self):
        return "({},{},{},{},{})".format(self.height, self.x, self.y, self.w, self.h)
    def __repr__(self):
        return self.__str__()
    def try_add_box(self, other):
        if self.h == MAX_CLUSTER_SIZE:
            return False
        if self.x != other.x:
            return False
        if self.y + self.h != other.y:
            return False
        if self.w != other.w:
            return False
        self.h += other.h
        return True
print("starting calculation with a max clustersize of 20")
print("parsing file " + RAW_FILE_PATH)
img_data=np.asarray(Image.open(RAW_FILE_PATH)) 

data = []
print("iteration 1: building boxes of format (1xN) with N =", MAX_CLUSTER_SIZE)
for y,row in enumerate(img_data):
    row_boxes = []
    for x, color in enumerate(row):
        was_added = False
        for box in row_boxes:
            if box.try_add_color(color,x,y):
                was_added = True
                break
        if not was_added:
            row_boxes.append(Box(color,x,y))
    data.extend(row_boxes)
max_offset = 80
delete_index = []
print("iteration 1 finished with ", len(data), " boxes")
print("iteration 2: merging boxes from previous iteration to (MxN)")
length = len(data)
for i in range(length):
    if i in delete_index:
        continue
    for offset in range(max_offset):
        if offset + i + 1 >= length:
            break
        if i + offset + 1 in delete_index:
            continue
        if data[i].y + data[i].h < data[i + offset + 1].y:
            break
        if data[i].try_add_box(data[i + offset + 1]):
            delete_index.append(i + offset + 1)
delete_index.sort(reverse=True)
for index in delete_index:
    data.pop(index)

print("iteration 2 finished with ", len(data), " boxes")
            
data_str = str(data)
data_str = "(" + data_str[:-1] + ("])")
print("writing boxes to " + OUTPUT_FILE_PATH)
text_file = open(OUTPUT_FILE_PATH, "w")
text_file.write(data_str)
text_file.close()
