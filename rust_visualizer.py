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

        print(pixel_size)

        # Ouvrir le fichier en mode binaire pour l'écriture
        with open(f"{path_to_rust_prog}/saves/save_{file_name}.bin", "wb") as file:
            print(pixel_size)
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
        command_to_execute = f"cd {path_to_rust_prog} && cargo run --release -- {filename}"
        print("command:", command_to_execute)
        subprocess.run(command_to_execute, shell=True,
                       stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)


if __name__ == "__main__":
    import pydicom
    import numpy as np

    # Charger le fichier DICOM
    dicom_file = pydicom.dcmread("temp.dcm")

    # Récupérer les données
    image_data = dicom_file.pixel_array

    # Créer un tableau 3D (hauteur, largeur, 4)
    shape = (image_data.shape[0], image_data.shape[1], 1, 4)
    image_3D = np.zeros(shape, dtype=np.uint8)

    # Remplir le tableau 3D
    image_3D[..., 0, :3] = image_data[..., None]
    image_3D[..., 0, 3] = 255

    print(image_3D.shape)

    # Nbr de pixel, les deux premiers nombres sont pour une coupe donnée, le 3eme est le nombre de couper
    nbr_of_pixels = (image_data.shape[0], image_data.shape[1], 1)

    # Espacement entre les pixels (x, y) en mm
    pixel_spacing_x, pixel_spacing_y = map(float, dicom_file.PixelSpacing)
    pixel_spacing_x = int(round(10.0 * pixel_spacing_x))
    pixel_spacing_y = int(round(10.0 * pixel_spacing_y))

    # Épaisseur de la slice en mm
    slice_thickness = int(round(dicom_file.SliceThickness * 10.0))

    # Taille des pixels en dixieme de mm (Pour l'instant on peut seulement avoir x=y): ici on a des pixel de 1mm de coté et 2mm entre deux slices
    pixel_size = (pixel_spacing_x, pixel_spacing_y, slice_thickness)

    # Path vers le dossier que je t'envois (en gros dossier qui contient le fichier "Cargo.toml")
    path = "."

    # Exemple pour créer un tableau 3d.
    rust_visualizer_object = RustVisualizer(nbr_of_pixels)
    rust_visualizer_object.array = image_3D

    rust_visualizer_object.start_visualizer(pixel_size, path)




