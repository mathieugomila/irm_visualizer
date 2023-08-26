import numpy as np
import struct

import subprocess

import datetime
import os

class RustVisualizer:
    array: np.array

    def __init__(self, nbr_of_pixel):
        shape = (nbr_of_pixel[0], nbr_of_pixel[1], nbr_of_pixel[2], 4)
        dtype = np.uint8
        self.array = np.zeros(shape, dtype=dtype)

    def export_to_bin(self, pixel_size, path_to_rust_prog):
        file_name = datetime.datetime.now().strftime("%Y-%m-%d-%H-%M-%S")

        os.makedirs("saves", exist_ok=True)

        # Ouvrir le fichier en mode binaire pour l'écriture
        with open(f"{path_to_rust_prog}/saves/save_{file_name}.bin", "wb") as file:
            data = struct.pack(
                'BBB', pixel_size[0], pixel_size[1], pixel_size[2])
            file.write(data)

            data = struct.pack(
                'BBB', self.array.shape[0], self.array.shape[1], self.array.shape[2])
            file.write(data)
            for x in range(self.array.shape[0]):
                for y in range(self.array.shape[1]):
                    for z in range(self.array.shape[2]):
                        # Utiliser la fonction struct.pack pour convertir les uint8 en format binaire
                        data = struct.pack(
                            'BBBB', self.array[x, y, z][0], self.array[x, y, z][1], self.array[x, y, z][2], self.array[x, y, z][3])

                        # Écrire les données dans le fichier binaire
                        file.write(data)
        return f"save_{file_name}"

    def start_visualizer(self, pixel_size, path_to_rust_prog):
        filename = self.export_to_bin(pixel_size, path_to_rust_prog)
        command_to_execute = f"cd {path_to_rust_prog} && cargo run -- {filename}"
        print("command:", command_to_execute)
        subprocess.run(command_to_execute, shell=True,
                       stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)


if __name__ == "__main__":
    # Nbr de pixel, les deux premiers nombres sont pour une coupe donnée, le 3eme est le nombre de couper
    nbr_of_pixels = (80, 80, 10)

    # Taille des pixels en dixieme de mm (Pour l'instant on peut seulement avoir x=y): ici on a des pixel de 1mm de coté et 2mm entre deux slices
    pixel_size = (10, 10, 60)

    # Path vers le dossier que je t'envois (en gros dossier qui contient le fichier "Cargo.toml")
    path = "."

    # Exemple pour créer un tableau 3d.
    rust_visualizer_object = RustVisualizer(nbr_of_pixels)
    # On parcourt le tableau 3d
    for x in range(nbr_of_pixels[0]):
        for y in range(nbr_of_pixels[1]):
            for z in range(nbr_of_pixels[2]):
                # Un pixel sur deux (c'est juste un exemple)
                if (x+y+z) % 2 == 0:
                    # Chaque élément de my_array[x, y, z] est un array de 4 élément :
                    #   - Le premier élément doit etre une valeur entre 0 et 255 inclus qui correspond à la couleur rouge
                    #   - Le deuxieme élément doit etre une valeur entre 0 et 255 inclus qui correspond à la couleur vert
                    #   - Le troisieme élément doit etre une valeur entre 0 et 255 inclus qui correspond à la couleur bleu
                    #   - Le dernier élément vaut 255 si on veut un cube, 0 sinon
                    #
                    # Si aucune valeur sont spécifié pour my_array[x, y, z] le cube est vide
                    rust_visualizer_object.array[x, y, z] = [int((float(x)/float(nbr_of_pixels[0])) * 255), int(
                        (float(y)/float(nbr_of_pixels[1])) * 255), int((float(z)/float(nbr_of_pixels[2])) * 255), 255]

    rust_visualizer_object.start_visualizer(pixel_size, path)
