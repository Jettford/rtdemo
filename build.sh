set -e

cd src

rm -rf ../isodir/
rm -rf ../bin/

mkdir -p ../isodir/boot/grub/
mkdir ../bin/

CSOURCE=$(find . -name '*.c')

for file in $CSOURCE; do
	../toolchain/bin/i686-elf-gcc -c $file -o ${file%.c}.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra
done

ASOURCE=$(find . -name '*.s')

for file in $ASOURCE; do
	../toolchain/bin/i686-elf-as $file -o ${file%.s}.o
done

RSSOURCE=$(find . -name '*.rs')

for file in $RSSOURCE; do
	rustc --target i686-unknown-uefi --crate-type=lib -O $file -o ${file%.rs}.o
done

OBJFILES=$(find . -name '*.o')

# Dumb warning ignore because I'm lazy
../toolchain/bin/i686-elf-gcc -T boot/linker.ld -o ../isodir/boot/rtdemo.bin -ffreestanding -O2 -nostdlib $OBJFILES -lgcc -Wl,--no-warn-rwx-segment

for file in $OBJFILES; do
	rm $file
done

# Some of these are sent to /dev/null to nuke the output, cba to make it look nice
grub-file --is-x86-multiboot ../isodir/boot/rtdemo.bin > /dev/null 2>&1
cp ../grub.cfg ../isodir/boot/grub/grub.cfg 
grub-mkrescue -o ../bin/rtdemo.iso ../isodir > /dev/null 2>&1

cp ../isodir/boot/rtdemo.bin ../bin/rtdemo.bin

rm -rf ../isodir/
