// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Brain Display

use core::ffi::{VaList, c_char};

use crate::platform::jump_table;

/// A decoded image written to by VEXos.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct v5_image {
    /// Definitive width of the output image.
    pub width: u16,

    /// Definitive height of the output image.
    pub height: u16,

    /// Buffer of RGB8 pixels that containing the image's data.
    ///
    /// This field must be set before the read operation as a pointer to the pre-allocated pixel buffer.
    /// After an image read operation, said image’s pixels are written to the location specified by this field.
    pub data: *mut u32,

    /// Points to the first pixel of the second row in the pixel buffer.
    ///
    /// Only set by the SDK after a [`vexImageBmpRead`] call.
    pub p: *mut u32,
}

jump_table! {
    /// Sets the color (encoded as RGB8) used for all future non-erasing display draws.
    0x640 => pub fn vexDisplayForegroundColor(col: u32);
    /// Sets the color (encoded as RGB8) used for all future erasing display draws.
    0x644 => pub fn vexDisplayBackgroundColor(col: u32);
    /// Fills the entire framebuffer with the current background color.
    0x648 => pub fn vexDisplayErase();
    /// Moves a region of the screen defined by all the pixels whose y-axis coordinate are
    /// within the range [nStartLine, 272) `nLines` pixels upwards, without affecting portions
    /// of the screen outside the specified scroll region.
    ///
    /// Since `nLines` is a signed integer, a negative value will move the pixels in the
    /// region downwards instead. Pixels that move outside the region being scrolled are
    /// discarded, and any portions of the region that no longer have a value after the
    /// operation are set to the background color.
    0x64c => pub fn vexDisplayScroll(nStartLine: i32, nLines: i32);
    0x69c => pub fn vexDisplayTextSmoothing(value: u32);
    0x6a0 => pub fn vexDisplayTextReference(value: u32);
    0x6ac => pub fn vexDisplayTextSpacing(value: u32);
    0x6a8 => pub fn vexDisplayTextSize(n: u32, d: u32);
    0x6b4 => pub fn vexDisplayFontNamedSet(pFontName: *const c_char);
    /// Gets the currently set foreground color as an RGB8 color.
    0x6b8 => pub fn vexDisplayForegroundColorGet() -> u32;
    /// Gets the currently set background color as an RGB8 color.
    0x6bc => pub fn vexDisplayBackgroundColorGet() -> u32;
    /// Returns the calculated width (in pixels) of a string if it were to be drawn to the display.
    ///
    /// This function uses the text size of the last text drawing operation for calculating width.
    0x6c0 => pub fn vexDisplayStringWidthGet(pString: *const c_char) -> i32;
    /// Returns the calculated height (in pixels) of a string if it were to be drawn to the display.
    ///
    /// This function uses the text size of the last text drawing operation for calculating height.
    0x6c4 => pub fn vexDisplayStringHeightGet(pString: *const c_char) -> i32;
    0x6c8 => pub fn vexDisplayPenSizeSet(value: u32);
    0x6cc => pub fn vexDisplayPenSizeGet() -> u32;
    /// Enables double-buffered mode on the display, flushing the intermediate framebuffer.
    ///
    /// The first time this function is called, double-buffered mode will be enabled. In order for future draws to be
    /// seen, this function will need to be called each frame to draw the secondary buffer to the display.
    ///
    /// To re-enable immediate-mode rendering (single-buffer), see [`vexDisplayDoubleBufferDisable`].
    ///
    /// # Arguments
    ///
    /// - `bVsyncWait`: Sleep the current task until the screen is ready to refresh.
    /// - `bRunScheduler`: Call [`vexTasksRun`](super::vexTasksRun) while waiting for a refresh.
    0x7a0 => pub fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool);
    /// Unimplemented as of VEXos 1.0.8.
    ///
    /// Disables double-buffered mode, switching back to immediate mode rendering.
    0x7a4 => pub fn vexDisplayDoubleBufferDisable();
    /// Decodes a bitmap-encoded image passed to `ibuf` into a buffer of pixels that can be drawn to the display.
    ///
    /// # Arguments
    ///
    /// - `ibuf`: The PNG file as a buffer of bytes.
    /// - `obuf`: A decoded image encoded as RGB8 pixels that will be written to if the operation succeeds.
    /// - `maxw`: Width capacity of the image buffer.
    /// - `maxh`: Height capacity of the image buffer.
    ///
    /// # Return
    ///
    /// `1` if the operation is successful, `0` if it failed.
    ///
    /// # Safety
    ///
    /// - `oBuf` must point to an initialized [`v5_image`] struct or null.
    /// - `(*oBuf).data` must point to a mutable allocated image buffer that is at least `maxw * maxh * 4` bytes long or be null.
    0x990 => pub fn vexImageBmpRead(
        ibuf: *const u8,
        oBuf: *mut v5_image,
        maxw: u32,
        maxh: u32,
    ) -> u32;
    /// Decodes a PNG file passed to `ibuf` into a buffer of pixels that can be drawn to the display. This function uses
    /// `libpng` internally to decode the file's contents.
    ///
    /// # Arguments
    ///
    /// - `ibuf`: The PNG file as a buffer of bytes.
    /// - `obuf`: A decoded image encoded as RGB8 pixels that will be written to if the operation succeeds.
    /// - `maxw`: Width capacity of the image buffer.
    /// - `maxh`: Height capacity of the image buffer.
    /// - `ibuflen`: Length of the input buffer.
    ///
    /// # Return
    ///
    /// `1` if the operation is successful, `0` if it failed.
    ///
    /// # Safety
    ///
    /// - `ibuf` must be null, OR point to a buffer of at least length ibuflen.
    /// - `oBuf` must point to an initialized [`v5_image`] struct or null.
    /// - `(*oBuf).data` must point to a mutable allocated image buffer that is at least `maxw * maxh * 4` bytes long or be null.
    0x994 => pub fn vexImagePngRead(
        ibuf: *const u8,
        oBuf: *mut v5_image,
        maxw: u32,
        maxh: u32,
        ibuflen: u32,
    ) -> u32;
    /// Draws a string of text to the display at a given line.
    ///
    /// Uses the current foreground color for the text itself, and the current background color if `bOpaque` is `true`.
    0x684 => pub fn vexDisplayVString(nLineNumber: i32, format: *const c_char, args: VaList);
    /// Draws a string of large-sized text to the display at a given line.
    ///
    /// Uses the current foreground color as the text color.
    0x68c => pub fn vexDisplayVBigString(nLineNumber: i32, format: *const c_char, args: VaList);
    /// Draws a string of center-justified text to the display at a given line.
    ///
    /// Uses the current foreground color as the text color.
    0x694 => pub fn vexDisplayVCenteredString(
        nLineNumber: i32,
        format: *const c_char,
        args: VaList,
    );
    /// Draws a string of large-sized, center-justified text to the display at a given line.
    ///
    /// Uses the current foreground color as the text color.
    0x698 => pub fn vexDisplayVBigCenteredString(
        nLineNumber: i32,
        format: *const c_char,
        args: VaList,
    );
    /// Moves a rectangular region of the screen `nLines` pixels upwards, without affecting
    /// portions of the screen outside the specified scroll region.
    ///
    /// Since `nLine` is a signed integer, a negative value will move the pixels in the
    /// region downwards instead. Pixels that move outside the region being scrolled are
    /// discarded, and any portions of the region that no longer have a value after the
    /// operation are set to the background color.
    ///
    /// # Bugs
    ///
    /// It appears that this function is somewhat bugged at the time of writing (on VEXos 1.1.4),
    /// as it will overwrite one too many lines, setting the bottommost row of scroll data to the
    /// background color.
    0x650 => pub fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32);
    /// Draw a buffer of pixels to a rectangular region of the screen.
    ///
    /// Each u32 element in the buffer is considered a pixel and is parsed in the same format
    /// used by vexDisplayForegroundColor (RGB8). The function allows you to specify the region
    /// to write to, the pointer to your image buffer, and the stride (or the number of u32
    /// pixels in your buffer per 1 row).
    0x654 => pub fn vexDisplayCopyRect(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        pSrc: *mut u32,
        srcStride: i32,
    );
    0x658 => pub fn vexDisplayPixelSet(x: u32, y: u32);
    0x65c => pub fn vexDisplayPixelClear(x: u32, y: u32);
    0x660 => pub fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32);
    0x664 => pub fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32);
    0x668 => pub fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32);
    0x66c => pub fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32);
    0x670 => pub fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32);
    0x674 => pub fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32);
    0x678 => pub fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32);
    0x67c => pub fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32);
    0x794 => pub fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32);
    0x7a8 => pub fn vexDisplayClipRegionSetWithIndex(
        index: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    );
    0x680 => pub fn vexDisplayVPrintf(
        xpos: i32,
        ypos: i32,
        bOpaque: i32,
        format: *const c_char,
        args: VaList,
    );
    0x688 => pub fn vexDisplayVStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList);
    0x690 => pub fn vexDisplayVBigStringAt(
        xpos: i32,
        ypos: i32,
        format: *const c_char,
        args: VaList,
    );
    0x6b0 => pub fn vexDisplayVSmallStringAt(
        xpos: i32,
        ypos: i32,
        format: *const c_char,
        args: VaList,
    );
}
