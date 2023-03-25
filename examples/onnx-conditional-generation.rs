use std::path::PathBuf;

use ort::ExecutionProvider;
use rust_bert::pipelines::common::{ModelResources, ModelType, ONNXModelResources};
use rust_bert::pipelines::generation_utils::{GenerateConfig, LanguageGenerator};
use rust_bert::pipelines::onnx::config::ONNXEnvironmentConfig;

use rust_bert::pipelines::onnx::models::ONNXConditionalGenerator;
use rust_bert::resources::LocalResource;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let onnx_causal_decoder = ONNXConditionalGenerator::new(
        GenerateConfig {
            model_type: ModelType::Marian,
            model_resource: ModelResources::ONNX(ONNXModelResources {
                encoder_resource: Some(Box::new(LocalResource::from(PathBuf::from(
                    "E:/Coding/opus-mt-en-fr-onnx/encoder_model.onnx",
                )))),
                decoder_resource: Some(Box::new(LocalResource::from(PathBuf::from(
                    "E:/Coding/opus-mt-en-fr-onnx/decoder_model.onnx",
                )))),
                decoder_with_past_resource: Some(Box::new(LocalResource::from(PathBuf::from(
                    "E:/Coding/opus-mt-en-fr-onnx/decoder_with_past_model.onnx",
                )))),
            }),
            config_resource: Box::new(LocalResource::from(PathBuf::from(
                "E:/Coding/opus-mt-en-fr-onnx/config.json",
            ))),
            vocab_resource: Box::new(LocalResource::from(PathBuf::from(
                "E:/Coding/opus-mt-en-fr-onnx/vocab.json",
            ))),
            merges_resource: Some(Box::new(LocalResource::from(PathBuf::from(
                "E:/Coding/opus-mt-en-fr-onnx/source.spm",
            )))),
            num_beams: 3,
            do_sample: false,
            ..Default::default()
        },
        None,
        Some(&ONNXEnvironmentConfig {
            execution_providers: Some(vec![ExecutionProvider::cuda()]),
            ..Default::default()
        }),
    )?;

    let output = onnx_causal_decoder.generate(Some(&["Hello my name is John."]), None);
    println!("{:?}", output);
    Ok(())
}
