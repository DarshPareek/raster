#ifndef COMMON_H_
#define COMMON_H_
#include <SDL2/SDL.h>
#include <SDL2/SDL_events.h>
#include <SDL2/SDL_mouse.h>
#include <SDL2/SDL_rect.h>
#include <SDL2/SDL_render.h>
#include <SDL2/SDL_timer.h>
#include <SDL2/SDL_video.h>
#include <complex.h>
#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wchar.h>

#define SCREEN_WIDTH 800
#define SCREEN_HEIGHT 800
#define SCREEN_FPS 60
#define DELTA_TIME_SEC (1.0f / SCREEN_FPS)
#define DELTA_TIME_MS (Uint32) floorf(DELTA_TIME_SEC * 1000.0f)
#define BACKGROUND_COLOR 0x091413FF
#define PART1_COLOR 0x408A71FF
#define PART2_COLOR 0x285A48FF
#define PART3_COLOR 0xB0E4CCFF
#define MARKER_SIZE 10
// clang-format off
#define HEX_COLOR(hex)                                                         \
  ((hex) >> (3 * 8)) & 0xFF,                                                   \
  ((hex) >> (2 * 8)) & 0xFF,                                                   \
  ((hex) >> (1 * 8)) & 0xFF,                                                   \
  ((hex) >> (0 * 8)) & 0xFF

// Common Checks for SDL Errors
int check_sdl_code(int code) {
  if (code < 0) {
    fprintf(stderr, "SDL error: %s\n", SDL_GetError());
    exit(1);
  }
  return code;
}
void *check_sdl_ptr(void *ptr) {
  if (ptr == NULL) {
    fprintf(stderr, "SDL error: %s\n", SDL_GetError());
    exit(1);
  }
  return ptr;
}
// LERP 1D
float lerpf(float a, float b, float p) { return a + (b - a) * p; }

// Vector Datatype and it's common functions
typedef struct {
  float x;
  float y;
} Vec2;

Vec2 vec2(float x, float y) { return (Vec2){x, y}; }
Vec2 vec2_sub(Vec2 a, Vec2 b) { return vec2(a.x - b.x, a.y - b.y); }
Vec2 vec2_scale(Vec2 a, float s) { return vec2(a.x * s, a.y * s); }
Vec2 vec2_add(Vec2 a, Vec2 b) { return vec2(a.x + b.x, a.y + b.y); }
Vec2 lerpv2(Vec2 a, Vec2 b, float p) { return vec2_add(a, vec2_scale(vec2_sub(b, a), p)); }
// clang-format on

#endif // COMMON_H_
