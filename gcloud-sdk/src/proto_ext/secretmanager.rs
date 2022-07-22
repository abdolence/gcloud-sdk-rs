use hyper::body::Buf;
use prost::bytes::BufMut;
use prost::encoding::{DecodeContext, WireType};
use prost::DecodeError;
use secret_vault_value::SecretValue;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct SecretPayload {
    pub data: SecretValue,

    pub data_crc32c: Option<i64>,
}

impl prost::Message for SecretPayload {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
        Self: Sized,
    {
        prost::encoding::bytes::encode(1, self.data.ref_sensitive_value(), buf);
        if let Some(ref crc32c) = self.data_crc32c {
            prost::encoding::int64::encode(2, crc32c, buf);
        }
    }

    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        Self: Sized,
    {
        match tag {
            1 => prost::encoding::bytes::merge(
                wire_type,
                self.data.ref_sensitive_value_mut(),
                buf,
                ctx,
            ),
            2 => prost::encoding::int64::merge(
                wire_type,
                self.data_crc32c.get_or_insert_with(Default::default),
                buf,
                ctx,
            ),
            _ => prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        prost::encoding::bytes::encoded_len(1, self.data.ref_sensitive_value())
            + self
                .data_crc32c
                .as_ref()
                .map_or(0, |crc32c| prost::encoding::int64::encoded_len(2, crc32c))
    }

    fn clear(&mut self) {
        self.data.secure_clear();
        self.data_crc32c = None;
    }
}
