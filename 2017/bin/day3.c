#include <math.h>
#include <stdio.h>

#define grid_size 50

int main() {
    int grid[2 * grid_size + 1][2 * grid_size + 1] = {0};
    grid[grid_size][grid_size] = 1;

    for (long i = 1; i <= grid_size; i++) {
        long side = 2 * i + 1;
        long steps = 8 * i;
        long x = grid_size + i;
        long y = grid_size + i - 1;

        for (long j = 0; j < steps; j++) {
            for (int dx = -1; dx <= 1; dx++) {
                for (int dy = -1; dy <= 1; dy++) {
                    if (!(dx == 0 && dy == 0)) {
                        grid[x][y] += grid[x + dx][y + dy];
                    }
                }
            }

            if (grid[x][y] > 347991) {
                printf("%d", grid[x][y]);
                return 0;
            }

            if (j < side - 2) {
                y -= 1;
            } else if (j < 2 * side - 3) {
                x -= 1;
            } else if (j < 3 * side - 4) {
                y += 1;
            } else {
                x += 1;
            }
        }
    }

    return 1;
}
