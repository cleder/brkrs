# Fallback Texture Placeholders

Drop PNG or KTX assets in this directory to serve as the canonical fallback visuals for each gameplay object class.
The default manifest references the following filenames:

- `ball_base.png`
- `paddle_base.png`
- `brick_base.png`
- `sidewall_base.png`
- `ground_base.png`
- `background_base.png`

These files can be low-resolution checkerboards; they simply guarantee every mesh renders with a textured material even when bespoke art is missing.
