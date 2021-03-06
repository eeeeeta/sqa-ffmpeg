//! Wrapper & iterator for the FFmpeg `AVFrame` type.
use ffmpeg_sys::*;
use errors::{MediaResult, MediaError};
use super::{SampleFormat, Sample};
use chrono::Duration;
use libc;
#[derive(Debug)]
pub struct Frame {
    ptr: *mut AVFrame,
    cur_chan: usize,
    cur_idx: usize,
    cap: usize,
    chans: usize,
    pts: libc::c_double,
    format: SampleFormat
}
impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            av_frame_free(&mut self.ptr);
        }
    }
}
impl Frame {
    pub unsafe fn from_ptr(ptr: *mut AVFrame, time: AVRational) -> MediaResult<Self> {
        let format = (*ptr).format;
        let format = if let Some(x) = SampleFormat::from_ffi(format) { x }
        else {
            Err(MediaError::UnsupportedFormat)?
        };
        let mut cap = (*ptr).nb_samples;
        let chans = (*ptr).channels;
        if !format.is_planar() {
            cap *= chans;
        }
        let pts = (*ptr).pts as libc::c_double;
        let time = av_q2d(time);
        Ok(Frame {
            ptr: ptr,
            cur_chan: 0,
            cur_idx: 0,
            format: format,
            pts: pts * time,
            cap: cap as usize,
            chans: chans as usize
        })
    }
    pub fn capacity(&self) -> usize {
        self.cap
    }
    pub fn drained(&self) -> bool {
        self.cur_idx >= self.cap
    }
    pub fn channels(&self) -> usize {
        self.chans
    }
    pub fn format(&self) -> SampleFormat {
        self.format
    }
    pub fn pts(&self) -> Duration {
        Duration::nanoseconds((1_000_000_000f64 * self.pts) as _)
    }
}
impl<'a> Iterator for &'a mut Frame {
    type Item = (usize, Sample);
    fn next(&mut self) -> Option<(usize, Sample)> {
        let chan = if self.format.is_planar() {
            self.cur_chan
        }
        else {
            0
        };
        if self.cur_idx >= self.cap {
            return None;
        }
        unsafe {
            let data = *(*self.ptr).extended_data.offset(chan as isize);
            let ret = match self.format {
                SampleFormat::U8(_) => Sample::U8(*(data as *mut u8).offset(self.cur_idx as isize)),
                SampleFormat::S16(_) => Sample::S16(*(data as *mut i16).offset(self.cur_idx as isize)),
                SampleFormat::S32(_) => Sample::S32(*(data as *mut i32).offset(self.cur_idx as isize)),
                SampleFormat::Float(_) => Sample::Float(*(data as *mut ::libc::c_float).offset(self.cur_idx as isize)),
                SampleFormat::Double(_) => Sample::Double(*(data as *mut ::libc::c_double).offset(self.cur_idx as isize)),
            };
            let ret = Some((self.cur_chan, ret));
            if self.format.is_planar() {
                self.cur_chan += 1;
                if self.cur_chan >= self.chans {
                    self.cur_chan = 0;
                    self.cur_idx += 1;
                }
            }
            else {
                self.cur_idx += 1;
                self.cur_chan += 1;
                if self.cur_chan >= self.chans {
                    self.cur_chan = 0;
                }
            }
            ret
        }
    }
}
