/*
 * Copyright(c) 2018 Free Software Foundation, Inc.
 *
 * This file is part of libwget.
 *
 * Libwget is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Libwget is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with libwget.  If not, see <https://www.gnu.org/licenses/>.
 */

#include <config.h>

#include <stdio.h>
#include <stdlib.h>

#include <wget.h>
#include "private.h"

/**
 * \file
 * \brief Functions for bitmap.
 *
 * @{
 *
 * Bitmap bit array manipulation functions.
 *
 */

#ifdef bitmap_64
#   define bitmap_type unsigned long long int
#   define bitmap_shift 6
#   define bitmap_mask 63
#   define bitmap_wordlength 64
#   define bitmap_fmt "%016llx"
#else //assume to be 32 bits
#   define bitmap_type unsigned int
#   define bitmap_shift 5
#   define bitmap_mask 31
#   define bitmap_wordlength 32
#   define bitmap_fmt "%08x"
#endif

// To get the bitmap_type right 
#define bitmap_one (bitmap_type)1

void wget_bitmap_set(wget_bitmap_t *b, int n)
{
    int word = n >> bitmap_shift; // n / bitmap_wordlength
    int position = n & bitmap_mask; // n % bitmap_wordlength
    b->array[word] |= bitmap_one << position;
}

void wget_bitmap_clear(wget_bitmap_t *b, int n)
{
    int word = n >> bitmap_shift; // n / bitmap_wordlength
    int position = n & bitmap_mask; // n % bitmap_wordlength
    b->array[word] &= ~(bitmap_one << position); 
}

void wget_bitmap_get(wget_bitmap_t *b, int n)
{
    int word = n >> bitmap_shift;
    int position = n & bitmap_mask;
    return (b->array[word] >> position) & 1;
}

wget_bitmap_t * wget_bitmap_allocate(int bits)
{
    wget_bitmap_t *b = malloc(sizeof(wget_bitmap_t));
    b->bits = bits;
    b->words = (bits + bitmap_wordlength - 1) / bitmap_wordlength;
        // divide, but round up for the ceiling
    b->array = calloc(b->words, sizeof(bitmap_type));
    return b;
}
                     
void wget_bitmap_deallocate(wget_bitmap_t *b)
{
    free(b->array);
    free(b);
}

