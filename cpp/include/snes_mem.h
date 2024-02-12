#ifndef __MEMORY_H
#define __MEMORY_H

#include <cstdint>
#include <vector>

namespace snes
{
    using Word = uint16_t;
    using Byte = uint8_t;
    using SnesPtr = uint32_t;
    using RawMem = std::vector<Byte>;
    const Word kWordMSB = 1 << 15;
    const Byte kByteMSB = 1 << 7;

    class SnesMemory
    {
    public:
        SnesMemory(void);

        // todo: change to template function
        void get(void *dest, SnesPtr src, int n_bytes);
        // void put(SnesPtr dest, void* src, int n_bytes);

        /**
         * @brief Read a word from memory.
         *
         * @param src Pointer to source.
         * @return Word
         */
        Word get_word(SnesPtr src);

        /**
         * @brief Read a byte from memory.
         *
         * @param src Pointer to source.
         * @return Byte
         */
        Byte get_byte(SnesPtr src);

        /**
         * @brief Write a word to memory.
         *
         * @param dest Address to destination.
         * @param val Value to be written.
         */
        void write(SnesPtr dest, Word val);

        /**
         * @brief Write a byte to memory.
         *
         * @param dest Address to destination.
         * @param val Value to be written.
         */
        void write(SnesPtr dest, Byte val);

        RawMem mem_;

    private:
    };
} // namespace snes

#endif