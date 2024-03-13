use image::ExtendedColorType;


    fn write_image(

        data: &[u8],
        width: u32,
        height: u32,
        color: ExtendedColorType,
    ) -> ImageResult<()> {
        let expected_buffer_len = color.buffer_size(width, height);
        assert_eq!(
            expected_buffer_len,
            data.len() as u64,
            "Invalid buffer length: expected {expected_buffer_len} got {} for {width}x{height} image",
            data.len(),
        );

        self.set_color(color);
        // `ravif` needs strongly typed data so let's convert. We can either use a temporarily
        // owned version in our own buffer or zero-copy if possible by using the input buffer.
        // This requires going through `rgb`.
        let mut fallback = vec![]; // This vector is used if we need to do a color conversion.
        let result = match Self::encode_as_img(&mut fallback, data, width, height, color)? {
            RgbColor::Rgb8(buffer) => self.encoder.encode_rgb(buffer),
            RgbColor::Rgba8(buffer) => self.encoder.encode_rgba(buffer),
        };
        let data = result.map_err(|err| {
            ImageError::Encoding(EncodingError::new(ImageFormat::Avif.into(), err))
        })?;
        self.inner.write_all(&data.avif_file)?;
        Ok(())
    }