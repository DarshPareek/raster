#include "./common.h"
#include <SDL2/SDL_events.h>
#include <SDL2/SDL_scancode.h>
#include <stddef.h>
#include <stdio.h>

#define PS_CAPACITY 256
Vec2 ps[PS_CAPACITY];
Vec2 xs[PS_CAPACITY];
size_t ps_count = 0;
int ps_selected = -1;
int animation = 0; // 0 off, 1 full, 2, bezier 3 parts
int markers = 0;   // 0 off, 1 on
int lines = 0;     // 0 off, 1 on
void fill_rect(SDL_Renderer *renderer, Vec2 pos, Vec2 size, uint32_t color) {
  check_sdl_code(SDL_SetRenderDrawColor(renderer, HEX_COLOR(color)));
  SDL_Rect rect = {(int)floorf(pos.x), (int)floorf(pos.y), (int)floorf(size.x),
                   (int)floorf(size.y)};
  check_sdl_code(SDL_RenderFillRect(renderer, &rect));
}

void render_line(SDL_Renderer *renderer, Vec2 begin, Vec2 end, uint32_t color) {
  check_sdl_code(SDL_SetRenderDrawColor(renderer, HEX_COLOR(color)));
  check_sdl_code(SDL_RenderDrawLine(renderer, (int)floorf(begin.x),
                                    (int)floorf(begin.y), (int)floorf(end.x),
                                    (int)floorf(end.y)));
}
void render_marker(SDL_Renderer *renderer, Vec2 pos, Vec2 marker_size,
                   uint32_t color) {
  fill_rect(renderer, vec2_sub(pos, vec2_scale(marker_size, 0.5)), marker_size,
            color);
}

Vec2 beziern_sample(Vec2 *xs, Vec2 *ps, size_t n, float p) {
  memcpy(xs, ps, sizeof(Vec2) * n);
  size_t i = 0;
  while (n > 1) {
    for (i = 0; i < n - 1; i++) {
      xs[i] = lerpv2(xs[i], xs[i + 1], p);
    }
    n--;
  }
  return xs[0];
}

void render_bezier_markers(SDL_Renderer *renderer, Vec2 *xs, Vec2 *ps, size_t n,
                           float s, Vec2 marker_size, uint32_t color) {
  float p = 0.0f;
  for (p = 0.0f; p < 1.0f; p += s) {
    render_marker(renderer, beziern_sample(xs, ps, n, p), marker_size, color);
  }
}

void render_bezier_curve(SDL_Renderer *renderer, Vec2 *xs, Vec2 *ps, size_t n,
                         float s, uint32_t color) {
  float p = 0.0f;
  for (p = 0.0f; p <= 1.0f; p += s) {
    Vec2 start = beziern_sample(xs, ps, n, p);
    Vec2 end = beziern_sample(xs, ps, n, p + s);
    render_line(renderer, start, end, color);
    start = end;
  }
}

int ps_at(Vec2 pos, Vec2 ps_size) {
  for (size_t i = 0; i < ps_count; i++) {
    const Vec2 ps_begin = vec2_sub(ps[i], vec2_scale(ps_size, 0.5f));
    const Vec2 ps_end = vec2_add(ps_begin, ps_size);
    if (ps_begin.x <= pos.x && pos.x <= ps_end.x && ps_begin.y <= pos.y &&
        pos.y <= ps_end.y) {
      return i;
    }
  }
  return -1;
}

int main(void) {
  check_sdl_code(SDL_Init(SDL_INIT_VIDEO));
  SDL_Window *const window =
      check_sdl_ptr(SDL_CreateWindow("Bezier Curves", 0, 0, SCREEN_WIDTH,
                                     SCREEN_HEIGHT, SDL_WINDOW_RESIZABLE));
  SDL_Renderer *const renderer =
      check_sdl_ptr(SDL_CreateRenderer(window, 0, SDL_RENDERER_ACCELERATED));
  int quit = 0;
  float t = 0.0f;
  float s = 0.1f;
  while (!quit) {
    SDL_Event event;
    while (SDL_PollEvent(&event)) {
      switch (event.type) {
      case SDL_QUIT: {
        quit = 1;
      } break;
      case SDL_MOUSEBUTTONDOWN: {
        switch (event.button.button) {
        case SDL_BUTTON_LEFT: {
          Vec2 mouse_pos = vec2(event.button.x, event.button.y);
          ps_selected = ps_at(mouse_pos, vec2(MARKER_SIZE, MARKER_SIZE));
          if (ps_selected < 0) {
            ps[ps_count++] = mouse_pos;
          }
        } break;
        }
        break;
      case SDL_MOUSEMOTION: {
        Vec2 mouse_pos = vec2(event.motion.x, event.motion.y);
        if (ps_selected > -1) {
          ps[ps_selected] = mouse_pos;
        }
      } break;
      case SDL_MOUSEWHEEL: {
        if (event.wheel.y > 0) {
          s += fmin(0.01f, 0.999f);
        } else if (event.wheel.y < 0) {
          s -= fmax(0.01f, 0.0001f);
        }
      } break;
      } break;
      case SDL_MOUSEBUTTONUP: {
        if (event.button.button == SDL_BUTTON_LEFT) {
          ps_selected = -1;
        }
      } break;
      case SDL_KEYDOWN: {
        switch (event.key.keysym.scancode) {
        case (SDL_SCANCODE_A): {
          animation = (animation + 1) % 3;
        } break;
        case (SDL_SCANCODE_S): {
          markers = (markers + 1) % 2;
        } break;
        case (SDL_SCANCODE_L): {
          lines = (lines + 1) % 2;
        } break;
        default: {
        } break;
        }
        break;
      }
      }
    }
    check_sdl_code(
        SDL_SetRenderDrawColor(renderer, HEX_COLOR(BACKGROUND_COLOR)));
    check_sdl_code(SDL_RenderClear(renderer));

    for (size_t i = 0; i < ps_count; i++) {
      if (markers == 1) {
        render_marker(renderer, ps[i], vec2(MARKER_SIZE, MARKER_SIZE),
                      PART1_COLOR);
      }
      if (i < ps_count - 1 && lines == 1) {
        render_line(renderer, ps[i], ps[i + 1], PART2_COLOR);
      }
      if (animation == 1) {
        for (size_t j = i; j < ps_count; j++) {
          if (i != j) {
            render_marker(renderer, lerpv2(ps[i], ps[j], (sinf(t) + 1) * 0.5f),
                          vec2(MARKER_SIZE, MARKER_SIZE), PART1_COLOR);
          }
        }
      }
    }

    if (ps_count >= 1) {
      render_bezier_curve(renderer, xs, ps, ps_count, s, PART3_COLOR);
    }
    if (animation == 2) {
      for (size_t i = 0; ps_count > 0 && i < ps_count - 1; i++) {
        render_marker(renderer, lerpv2(ps[i], ps[i + 1], (sinf(t) + 1) * 0.5f),
                      vec2(MARKER_SIZE, MARKER_SIZE), PART2_COLOR);
      }
      for (size_t i = 0; ps_count > 1 && i < ps_count - 2; i++) {
        Vec2 a = lerpv2(ps[i], ps[i + 1], (sinf(t) + 1) * 0.5f);
        Vec2 b = lerpv2(ps[i + 1], ps[i + 2], (sinf(t) + 1) * 0.5f);
        render_marker(renderer, lerpv2(a, b, (sinf(t) + 1) * 0.5f),
                      vec2(MARKER_SIZE, MARKER_SIZE), PART3_COLOR);
      }
    }

    SDL_RenderPresent(renderer);
    check_sdl_code(
        SDL_RenderSetLogicalSize(renderer, SCREEN_WIDTH, SCREEN_HEIGHT));
    SDL_Delay(DELTA_TIME_MS);
    t += DELTA_TIME_SEC;
  }
  SDL_Quit();
  return 0;
}
