// #include <SDL/SDL_error.h>
// #include <SDL/SDL_events.h>
#include <SDL2/SDL.h>
#include <SDL2/SDL_events.h>
#include <SDL2/SDL_mouse.h>
#include <SDL2/SDL_rect.h>
#include <SDL2/SDL_render.h>
#include <SDL2/SDL_timer.h>
#include <SDL2/SDL_video.h>
#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
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
#define HEX_COLOR(hex)                                                         \
  ((hex) >> (3 * 8)) & 0xFF, ((hex) >> (2 * 8)) & 0xFF,                        \
      ((hex) >> (1 * 8)) & 0xFF, ((hex) >> (0 * 8)) & 0xFF
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

float lerpf(float a, float b, float p) { return a + (b - a) * p; }

typedef struct {
  float x;
  float y;
} Vec2;

Vec2 vec2(float x, float y) { return (Vec2){x, y}; }

Vec2 vec2_sub(Vec2 a, Vec2 b) { return vec2(a.x - b.x, a.y - b.y); }
Vec2 vec2_scale(Vec2 a, float s) { return vec2(a.x * s, a.y * s); }
Vec2 vec2_add(Vec2 a, Vec2 b) { return vec2(a.x + b.x, a.y + b.y); }
Vec2 lerpv2(Vec2 a, Vec2 b, float p) {
  return vec2_add(a, vec2_scale(vec2_sub(b, a), p));
}

void render_line(SDL_Renderer *renderer, Vec2 begin, Vec2 end, uint32_t color) {
  check_sdl_code(SDL_SetRenderDrawColor(renderer, HEX_COLOR(color)));
  check_sdl_code(SDL_RenderDrawLine(renderer, (int)floorf(begin.x),
                                    (int)floorf(begin.y), (int)floorf(end.x),
                                    (int)floorf(end.y)));
}
void fill_rect(SDL_Renderer *renderer, Vec2 pos, Vec2 size, uint32_t color) {
  check_sdl_code(SDL_SetRenderDrawColor(renderer, HEX_COLOR(color)));
  SDL_Rect rect = {(int)floorf(pos.x), (int)floorf(pos.y), (int)floorf(size.x),
                   (int)floorf(size.y)};
  check_sdl_code(SDL_RenderFillRect(renderer, &rect));
}
void render_marker(SDL_Renderer *renderer, Vec2 pos, Vec2 marker_size,
                   uint32_t color) {
  fill_rect(renderer, vec2_sub(pos, vec2_scale(marker_size, 0.5)), marker_size,
            color);
}

void render_bezier_markers(SDL_Renderer *renderer, Vec2 a, Vec2 b, Vec2 c,
                           float s, Vec2 marker_size, uint32_t color) {
  float p = 0.0f;
  for (p = 0.0f; p < 1.0f; p += s) {
    Vec2 ab = lerpv2(a, b, p);
    Vec2 bc = lerpv2(b, c, p);
    Vec2 abc = lerpv2(ab, bc, p);
    render_marker(renderer, abc, marker_size, color);
  }
}

#define PS_CAPACITY 256

Vec2 ps[PS_CAPACITY];
size_t ps_count = 0;

int main(void) {
  check_sdl_code(SDL_Init(SDL_INIT_VIDEO));
  Vec2 marker_size = {20, 20};
  SDL_Window *const window =
      check_sdl_ptr(SDL_CreateWindow("Bezier Curves", 0, 0, SCREEN_WIDTH,
                                     SCREEN_HEIGHT, SDL_WINDOW_RESIZABLE));
  SDL_Renderer *const renderer =
      check_sdl_ptr(SDL_CreateRenderer(window, 0, SDL_RENDERER_ACCELERATED));
  float p = 0;
  int quit = 0;
  float t = 0.0f;
  while (!quit) {
    SDL_Event event;
    while (SDL_PollEvent(&event)) {
      switch (event.type) {
      case SDL_QUIT:
        quit = 1;
        break;
      case SDL_MOUSEBUTTONDOWN:
        switch (event.button.button) {
        case SDL_BUTTON_LEFT:
          ps[ps_count++] = vec2(event.button.x, event.button.y);
          break;
        case SDL_BUTTON_RIGHT:
          break;
        }
        break;
      }
    }
    check_sdl_code(
        SDL_SetRenderDrawColor(renderer, HEX_COLOR(BACKGROUND_COLOR)));
    check_sdl_code(SDL_RenderClear(renderer));

    for (size_t i = 0; ps_count > 0 && i < ps_count; i++) {
      render_marker(renderer, ps[i], marker_size, PART1_COLOR);
    }

    for (size_t i = 0; ps_count > 2 && i < ps_count - 3; i++) {
      // if (ps_count >= 3) {
      render_bezier_markers(renderer, ps[i], ps[i + 1], ps[i + 2], 0.001f,
                            marker_size, PART2_COLOR);
    }

    // Bezier Animations
    // PART 1
    // for (size_t i = 0; ps_count > 0 && i < ps_count; i++) {
    //   render_marker(renderer, ps[i], marker_size, PART1_COLOR);
    // }

    // // COOL ANIMATION VERY USELESS
    // // for (size_t i = 0; i < ps_count; i++) {
    // //   for (size_t j = i; j < ps_count; j++) {
    // //     if (i != j) {
    // //       render_marker(renderer, lerpv2(ps[i], ps[j], (sinf(t) + 1) *
    // 0.5f),
    // //                     marker_size, PART1_COLOR);
    // //     }
    // //   }
    // // }

    // // PART 2
    // for (size_t i = 0; ps_count > 0 && i < ps_count - 1; i++) {
    //   render_marker(renderer, lerpv2(ps[i], ps[i + 1], (sinf(t) + 1) * 0.5f),
    //                 marker_size, PART2_COLOR);
    // }

    // // PART 3
    // for (size_t i = 0; ps_count > 1 && i < ps_count - 2; i++) {
    //   Vec2 a = lerpv2(ps[i], ps[i + 1], (sinf(t) + 1) * 0.5f);
    //   Vec2 b = lerpv2(ps[i + 1], ps[i + 2], (sinf(t) + 1) * 0.5f);
    //   render_marker(renderer, lerpv2(a, b, (sinf(t) + 1) * 0.5f),
    //   marker_size,
    //                 PART3_COLOR);
    // }

    SDL_RenderPresent(renderer);
    check_sdl_code(
        SDL_RenderSetLogicalSize(renderer, SCREEN_WIDTH, SCREEN_HEIGHT));
    SDL_Delay(DELTA_TIME_MS);
    t += DELTA_TIME_SEC;
  }

  SDL_Quit();
  return 0;
}
