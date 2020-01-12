typedef struct {
	int x;
	int y;
} refs_interface;

refs_interface refs_instance;

int main() {

	return refs_instance.x + 1337;
}

