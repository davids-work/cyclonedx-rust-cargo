/*
 * This file is part of CycloneDX Rust Cargo.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use serde::{Deserialize, Serialize};
use xml::{
    name::OwnedName,
    reader::{self},
    writer,
};

use crate::{
    errors::XmlReadError,
    models::{self, bom::BomReference},
    prelude::Uri,
    specs::common::{
        organization::{OrganizationalContact, OrganizationalEntity},
        property::Properties,
    },
    utilities::{convert_optional, convert_vec},
    xml::{
        optional_attribute, read_list_tag, read_simple_tag, to_xml_read_error, to_xml_write_error,
        unexpected_element_error, write_close_tag, write_simple_tag, write_start_tag, FromXml,
        ToInnerXml, ToXml,
    },
};

use super::attachment::Attachment;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModelCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bom_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) model_parameters: Option<ModelParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) quantitative_analysis: Option<QuantitativeAnalysis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) considerations: Option<Considerations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) properties: Option<Properties>,
}

impl From<models::modelcard::ModelCard> for ModelCard {
    fn from(other: models::modelcard::ModelCard) -> Self {
        Self {
            bom_ref: other.bom_ref.map(|r| r.0),
            model_parameters: convert_optional(other.model_parameters),
            quantitative_analysis: convert_optional(other.quantitative_analysis),
            considerations: convert_optional(other.considerations),
            properties: convert_optional(other.properties),
        }
    }
}

impl From<ModelCard> for models::modelcard::ModelCard {
    fn from(other: ModelCard) -> Self {
        Self {
            bom_ref: other.bom_ref.map(BomReference::new),
            model_parameters: convert_optional(other.model_parameters),
            quantitative_analysis: convert_optional(other.quantitative_analysis),
            considerations: convert_optional(other.considerations),
            properties: convert_optional(other.properties),
        }
    }
}

const MODEL_CARD: &str = "modelCard";
const MODEL_PARAMETERS_TAG: &str = "modelParameters";
const BOM_REF_ATTR: &str = "bom-ref";

impl ToXml for ModelCard {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        let mut model_card_start_tag = writer::XmlEvent::start_element(MODEL_CARD);
        if let Some(bom_ref) = &self.bom_ref {
            model_card_start_tag = model_card_start_tag.attr("bom-ref", bom_ref);
        }
        writer
            .write(model_card_start_tag)
            .map_err(to_xml_write_error(MODEL_CARD))?;

        if let Some(model_parameters) = &self.model_parameters {
            model_parameters.write_xml_element(writer)?;
        }

        if let Some(quantitative_analysis) = &self.quantitative_analysis {
            quantitative_analysis.write_xml_element(writer)?;
        }

        if let Some(considerations) = &self.considerations {
            considerations.write_xml_element(writer)?;
        }

        if let Some(properties) = &self.properties {
            properties.write_xml_element(writer)?;
        }

        write_close_tag(writer, MODEL_CARD)?;

        Ok(())
    }
}

impl FromXml for ModelCard {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &xml::name::OwnedName,
        attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let bom_ref = optional_attribute(attributes, BOM_REF_ATTR);
        let mut model_parameters: Option<ModelParameters> = None;
        let mut quantitative_analysis: Option<QuantitativeAnalysis> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == MODEL_PARAMETERS_TAG => {
                    model_parameters = Some(ModelParameters::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?)
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == QUANTITATIVE_ANALYSIS_TAG => {
                    quantitative_analysis = Some(QuantitativeAnalysis::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self {
            bom_ref,
            model_parameters,
            quantitative_analysis,
            considerations: None,
            properties: None,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModelParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) approach: Option<ModelParametersApproach>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) architecture_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) model_architecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) datasets: Option<Datasets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) inputs: Option<Inputs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outputs: Option<Outputs>,
}

impl From<models::modelcard::ModelParameters> for ModelParameters {
    fn from(other: models::modelcard::ModelParameters) -> Self {
        Self {
            approach: convert_optional(other.approach),
            task: other.task,
            architecture_family: other.architecture_family,
            model_architecture: other.model_architecture,
            datasets: convert_optional(other.datasets),
            inputs: convert_optional(other.inputs),
            outputs: convert_optional(other.outputs),
        }
    }
}

impl From<ModelParameters> for models::modelcard::ModelParameters {
    fn from(other: ModelParameters) -> Self {
        Self {
            approach: convert_optional(other.approach),
            task: other.task,
            architecture_family: other.architecture_family,
            model_architecture: other.model_architecture,
            datasets: convert_optional(other.datasets),
            inputs: convert_optional(other.inputs),
            outputs: convert_optional(other.outputs),
        }
    }
}

const APPROACH_TAG: &str = "approach";
const TASK_TAG: &str = "task";
const ARCHITECTURE_FAMILY_TAG: &str = "architectureFamily";
const MODEL_ARCHITECTURE_TAG: &str = "modelArchitecture";
const INPUTS_TAG: &str = "inputs";
const INPUT_TAG: &str = "input";
const OUTPUTS_TAG: &str = "outputs";
const OUTPUT_TAG: &str = "output";
const FORMAT_TAG: &str = "format";
const ATTACHMENT_TAG: &str = "attachment";

impl ToXml for ModelParameters {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, MODEL_PARAMETERS_TAG)?;

        if let Some(approach) = &self.approach {
            approach.write_xml_element(writer)?;
        }

        if let Some(task) = &self.task {
            write_simple_tag(writer, TASK_TAG, task)?;
        }

        if let Some(architecture_family) = &self.architecture_family {
            write_simple_tag(writer, ARCHITECTURE_FAMILY_TAG, architecture_family)?;
        }

        if let Some(model_architecture) = &self.model_architecture {
            write_simple_tag(writer, MODEL_ARCHITECTURE_TAG, model_architecture)?;
        }

        if let Some(datasets) = &self.datasets {
            datasets.write_xml_element(writer)?;
        }

        if let Some(inputs) = &self.inputs {
            inputs.write_xml_element(writer)?;
        }

        if let Some(outputs) = &self.outputs {
            outputs.write_xml_element(writer)?;
        }

        write_close_tag(writer, MODEL_PARAMETERS_TAG)?;

        Ok(())
    }
}

impl FromXml for ModelParameters {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &xml::name::OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut approach: Option<ModelParametersApproach> = None;
        let mut task: Option<String> = None;
        let mut architecture_family: Option<String> = None;
        let mut model_architecture: Option<String> = None;
        let mut datasets: Option<Datasets> = None;
        let mut inputs: Option<Inputs> = None;
        let mut outputs: Option<Outputs> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == APPROACH_TAG => {
                    approach = Some(ModelParametersApproach::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?)
                }

                reader::XmlEvent::StartElement { name, .. } if name.local_name == TASK_TAG => {
                    task = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement { name, .. }
                    if name.local_name == ARCHITECTURE_FAMILY_TAG =>
                {
                    architecture_family = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement { name, .. }
                    if name.local_name == MODEL_ARCHITECTURE_TAG =>
                {
                    model_architecture = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == DATASETS_TAG => {
                    datasets = Some(Datasets::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == INPUTS_TAG => {
                    inputs = Some(Inputs::read_xml_element(event_reader, &name, &attributes)?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == OUTPUTS_TAG => {
                    outputs = Some(Outputs::read_xml_element(event_reader, &name, &attributes)?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self {
            approach,
            task,
            architecture_family,
            model_architecture,
            datasets,
            inputs,
            outputs,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModelParametersApproach {
    #[serde(rename = "type")]
    pub(crate) approach_type: Option<String>,
}

impl From<models::modelcard::ModelParametersApproach> for ModelParametersApproach {
    fn from(other: models::modelcard::ModelParametersApproach) -> Self {
        Self {
            approach_type: other.approach_type.map(|at| at.to_string()),
        }
    }
}

impl From<ModelParametersApproach> for models::modelcard::ModelParametersApproach {
    fn from(other: ModelParametersApproach) -> Self {
        Self {
            approach_type: other
                .approach_type
                .map(models::modelcard::ApproachType::new_unchecked),
        }
    }
}

const TYPE_TAG: &str = "type";

impl ToXml for ModelParametersApproach {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, APPROACH_TAG)?;

        if let Some(approach_type) = &self.approach_type {
            write_simple_tag(writer, TYPE_TAG, approach_type)?;
        }

        write_close_tag(writer, APPROACH_TAG)?;
        Ok(())
    }
}

impl FromXml for ModelParametersApproach {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &xml::name::OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut approach_type: Option<String> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement { name, .. } if name.local_name == TYPE_TAG => {
                    approach_type = Some(read_simple_tag(event_reader, &name)?)
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self { approach_type })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub(crate) struct Datasets(pub Vec<Dataset>);

impl From<models::modelcard::Datasets> for Datasets {
    fn from(other: models::modelcard::Datasets) -> Self {
        Datasets(convert_vec(other.0))
    }
}

impl From<Datasets> for models::modelcard::Datasets {
    fn from(other: Datasets) -> Self {
        models::modelcard::Datasets(convert_vec(other.0))
    }
}

impl ToXml for Datasets {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, DATASETS_TAG)?;

        for dataset in &self.0 {
            dataset.write_xml_element(writer)?;
        }

        write_close_tag(writer, DATASETS_TAG)?;

        Ok(())
    }
}

impl FromXml for Datasets {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut datasets = Vec::new();

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == DATASET_TAG => {
                    datasets.push(Dataset::read_xml_element(event_reader, &name, &attributes)?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                unexpected => return Err(unexpected_element_error(element_name, unexpected)),
            }
        }

        Ok(Self(datasets))
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum Dataset {
    Component(ComponentData),
    Reference(String),
}

impl From<models::modelcard::Dataset> for Dataset {
    fn from(other: models::modelcard::Dataset) -> Self {
        match other {
            models::modelcard::Dataset::Component(component) => Self::Component(component.into()),
            models::modelcard::Dataset::Reference(reference) => Self::Reference(reference),
        }
    }
}

impl From<Dataset> for models::modelcard::Dataset {
    fn from(other: Dataset) -> Self {
        match other {
            Dataset::Component(component) => {
                models::modelcard::Dataset::Component(component.into())
            }
            Dataset::Reference(reference) => models::modelcard::Dataset::Reference(reference),
        }
    }
}

const DATASETS_TAG: &str = "datasets";
const DATASET_TAG: &str = "dataset";
const CONTENTS_TAG: &str = "contents";
const GRAPHICS_TAG: &str = "graphics";
const NAME_TAG: &str = "name";
const CLASSIFICATION_TAG: &str = "classification";
const SENSITIVE_DATA_TAG: &str = "sensitiveData";
const GOVERNANCE_TAG: &str = "governance";
const REF_TAG: &str = "ref";

impl ToXml for Dataset {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        match self {
            Dataset::Component(component) => {
                component.write_xml_element(writer)?;
            }
            Dataset::Reference(reference) => {
                write_start_tag(writer, DATASET_TAG)?;
                write_simple_tag(writer, REF_TAG, reference)?;
                write_close_tag(writer, DATASET_TAG)?;
            }
        }

        Ok(())
    }
}

impl FromXml for Dataset {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &xml::name::OwnedName,
        attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let bom_ref = optional_attribute(attributes, BOM_REF_ATTR);
        let mut data_type: Option<String> = None;
        let mut data_name: Option<String> = None;
        let mut contents: Option<DataContents> = None;
        let mut classification: Option<String> = None;
        let mut graphics: Option<Graphics> = None;
        let mut description: Option<String> = None;
        let mut governance: Option<DataGovernance> = None;
        let mut sensitive_data: Option<String> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(DATASET_TAG))?;

            match next_element {
                reader::XmlEvent::StartElement { name, .. } if name.local_name == TYPE_TAG => {
                    data_type = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement { name, .. } if name.local_name == NAME_TAG => {
                    data_name = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == CONTENTS_TAG => {
                    contents = Some(DataContents::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::StartElement { name, .. }
                    if name.local_name == DESCRIPTION_TAG =>
                {
                    description = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement { name, .. }
                    if name.local_name == CLASSIFICATION_TAG =>
                {
                    classification = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == GOVERNANCE_TAG => {
                    governance = Some(DataGovernance::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == GRAPHICS_TAG => {
                    graphics = Some(Graphics::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::StartElement { name, .. }
                    if name.local_name == SENSITIVE_DATA_TAG =>
                {
                    sensitive_data = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        let data_type = data_type.ok_or_else(|| XmlReadError::RequiredDataMissing {
            required_field: TYPE_TAG.to_string(),
            element: element_name.local_name.to_string(),
        })?;

        Ok(Self::Component(ComponentData {
            bom_ref,
            data_type,
            name: data_name,
            contents,
            classification,
            sensitive_data,
            graphics,
            description,
            governance,
        }))
    }
}

/// Dataset component, for more details see:
/// https://cyclonedx.org/docs/1.5/json/#tab-pane_components_items_modelCard_modelParameters_datasets_items_oneOf_i1
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ComponentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bom_ref: Option<String>,
    #[serde(rename = "type")]
    pub(crate) data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) contents: Option<DataContents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) classification: Option<String>,
    /// Marked as an array of `String`, but examples use a single entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sensitive_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) graphics: Option<Graphics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) governance: Option<DataGovernance>,
}

impl From<models::modelcard::ComponentData> for ComponentData {
    fn from(other: models::modelcard::ComponentData) -> Self {
        Self {
            bom_ref: other.bom_ref.map(|r| r.0),
            data_type: other.data_type.to_string(),
            name: other.name,
            contents: convert_optional(other.contents),
            classification: convert_optional(other.classification),
            sensitive_data: convert_optional(other.sensitive_data),
            graphics: convert_optional(other.graphics),
            description: convert_optional(other.description),
            governance: convert_optional(other.governance),
        }
    }
}

impl From<ComponentData> for models::modelcard::ComponentData {
    fn from(other: ComponentData) -> Self {
        Self {
            bom_ref: other.bom_ref.map(BomReference::new),
            data_type: models::modelcard::ComponentDataType::new_unchecked(other.data_type),
            name: other.name,
            contents: convert_optional(other.contents),
            classification: convert_optional(other.classification),
            sensitive_data: convert_optional(other.sensitive_data),
            graphics: convert_optional(other.graphics),
            description: convert_optional(other.description),
            governance: convert_optional(other.governance),
        }
    }
}

impl ToXml for ComponentData {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        let mut start_tag = writer::XmlEvent::start_element(DATASET_TAG);
        if let Some(bom_ref) = &self.bom_ref {
            start_tag = start_tag.attr(BOM_REF_ATTR, bom_ref);
        }
        writer
            .write(start_tag)
            .map_err(to_xml_write_error(DATASET_TAG))?;

        if let Some(name) = &self.name {
            write_simple_tag(writer, NAME_TAG, name)?;
        }

        if let Some(contents) = &self.contents {
            contents.write_xml_element(writer)?;
        }

        if let Some(classification) = &self.classification {
            write_simple_tag(writer, CLASSIFICATION_TAG, classification)?;
        }

        if let Some(sensitive_data) = &self.sensitive_data {
            write_simple_tag(writer, SENSITIVE_DATA_TAG, sensitive_data)?;
        }

        if let Some(graphics) = &self.graphics {
            graphics.write_xml_element(writer)?;
        }

        if let Some(description) = &self.description {
            write_simple_tag(writer, DESCRIPTION_TAG, description)?;
        }

        if let Some(governance) = &self.governance {
            governance.write_xml_element(writer)?;
        }

        write_close_tag(writer, DATASET_TAG)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DataContents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) attachment: Option<Attachment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) properties: Option<Properties>,
}

impl From<models::modelcard::DataContents> for DataContents {
    fn from(other: models::modelcard::DataContents) -> Self {
        Self {
            attachment: convert_optional(other.attachment),
            url: other.url.map(|url| url.to_string()),
            properties: convert_optional(other.properties),
        }
    }
}

impl From<DataContents> for models::modelcard::DataContents {
    fn from(other: DataContents) -> Self {
        Self {
            attachment: convert_optional(other.attachment),
            url: other.url.map(Uri),
            properties: convert_optional(other.properties),
        }
    }
}

impl ToXml for DataContents {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, CONTENTS_TAG)?;

        if let Some(attachment) = &self.attachment {
            attachment.write_xml_named_element(writer, ATTACHMENT_TAG)?;
        }

        if let Some(url) = &self.url {
            write_simple_tag(writer, URL_TAG, url)?;
        }

        if let Some(properties) = &self.properties {
            properties.write_xml_element(writer)?;
        }

        write_close_tag(writer, CONTENTS_TAG)?;

        Ok(())
    }
}

const URL_TAG: &str = "url";
const PROPERTIES_TAG: &str = "properties";

impl FromXml for DataContents {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut url: Option<String> = None;
        let mut attachment: Option<Attachment> = None;
        let mut properties: Option<Properties> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement { name, .. } if name.local_name == URL_TAG => {
                    url = Some(read_simple_tag(event_reader, &name)?)
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == ATTACHMENT_TAG => {
                    attachment = Some(Attachment::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == PROPERTIES_TAG => {
                    properties = Some(Properties::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?)
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self {
            attachment,
            url,
            properties,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QuantitativeAnalysis {
    pub(crate) performance_metrics: Option<PerformanceMetrics>,
    pub(crate) graphics: Option<Graphics>,
}

impl From<models::modelcard::QuantitativeAnalysis> for QuantitativeAnalysis {
    fn from(other: models::modelcard::QuantitativeAnalysis) -> Self {
        Self {
            performance_metrics: convert_optional(other.performance_metrics),
            graphics: convert_optional(other.graphics),
        }
    }
}

impl From<QuantitativeAnalysis> for models::modelcard::QuantitativeAnalysis {
    fn from(other: QuantitativeAnalysis) -> Self {
        Self {
            performance_metrics: convert_optional(other.performance_metrics),
            graphics: convert_optional(other.graphics),
        }
    }
}

const QUANTITATIVE_ANALYSIS_TAG: &str = "quantitativeAnalysis";
const PERFORMANCE_METRICS_TAG: &str = "performanceMetrics";
const PERFORMANCE_METRIC_TAG: &str = "performanceMetric";

impl ToXml for QuantitativeAnalysis {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, QUANTITATIVE_ANALYSIS_TAG)?;

        if let Some(performance_metrics) = &self.performance_metrics {
            performance_metrics.write_xml_element(writer)?;
        }

        if let Some(graphics) = &self.graphics {
            graphics.write_xml_element(writer)?;
        }

        write_close_tag(writer, QUANTITATIVE_ANALYSIS_TAG)?;

        Ok(())
    }
}

impl FromXml for QuantitativeAnalysis {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut performance_metrics: Option<PerformanceMetrics> = None;
        let mut graphics: Option<Graphics> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == PERFORMANCE_METRICS_TAG => {
                    performance_metrics = Some(PerformanceMetrics::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == GRAPHICS_TAG => {
                    graphics = Some(Graphics::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self {
            performance_metrics,
            graphics,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PerformanceMetrics(pub(crate) Vec<PerformanceMetric>);

impl From<PerformanceMetrics> for models::modelcard::PerformanceMetrics {
    fn from(other: PerformanceMetrics) -> Self {
        Self(convert_vec(other.0))
    }
}

impl From<models::modelcard::PerformanceMetrics> for PerformanceMetrics {
    fn from(other: models::modelcard::PerformanceMetrics) -> Self {
        Self(convert_vec(other.0))
    }
}

impl ToXml for PerformanceMetrics {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, PERFORMANCE_METRICS_TAG)?;

        for metric in self.0.iter() {
            metric.write_xml_element(writer)?;
        }

        write_close_tag(writer, PERFORMANCE_METRICS_TAG)?;

        Ok(())
    }
}

impl FromXml for PerformanceMetrics {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut metrics = Vec::new();

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == PERFORMANCE_METRIC_TAG => {
                    metrics.push(PerformanceMetric::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self(metrics))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct PerformanceMetric {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub(crate) metric_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) slice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confidence_interval: Option<ConfidenceInterval>,
}

impl From<PerformanceMetric> for models::modelcard::PerformanceMetric {
    fn from(other: PerformanceMetric) -> Self {
        Self {
            metric_type: convert_optional(other.metric_type),
            value: convert_optional(other.value),
            slice: convert_optional(other.slice),
            confidence_interval: convert_optional(other.confidence_interval),
        }
    }
}

impl From<models::modelcard::PerformanceMetric> for PerformanceMetric {
    fn from(other: models::modelcard::PerformanceMetric) -> Self {
        Self {
            metric_type: convert_optional(other.metric_type),
            value: convert_optional(other.value),
            slice: convert_optional(other.slice),
            confidence_interval: convert_optional(other.confidence_interval),
        }
    }
}

const VALUE_TAG: &str = "value";
const SLICE_TAG: &str = "slice";
const CONFIDENCE_INTERVAL_TAG: &str = "confidenceInterval";

impl ToXml for PerformanceMetric {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, PERFORMANCE_METRIC_TAG)?;

        if let Some(metric_type) = &self.metric_type {
            write_simple_tag(writer, TYPE_TAG, metric_type)?;
        }

        if let Some(value) = &self.value {
            write_simple_tag(writer, VALUE_TAG, value)?;
        }

        if let Some(slice) = &self.slice {
            write_simple_tag(writer, SLICE_TAG, slice)?;
        }

        if let Some(confidence_interval) = &self.confidence_interval {
            confidence_interval.write_xml_element(writer)?;
        }

        write_close_tag(writer, PERFORMANCE_METRIC_TAG)?;

        Ok(())
    }
}

impl FromXml for PerformanceMetric {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut metric_type: Option<String> = None;
        let mut value: Option<String> = None;
        let mut slice: Option<String> = None;
        let mut confidence_interval: Option<ConfidenceInterval> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement { name, .. } if name.local_name == TYPE_TAG => {
                    metric_type = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement { name, .. } if name.local_name == VALUE_TAG => {
                    value = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement { name, .. } if name.local_name == SLICE_TAG => {
                    slice = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == CONFIDENCE_INTERVAL_TAG => {
                    confidence_interval = Some(ConfidenceInterval::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self {
            metric_type,
            value,
            slice,
            confidence_interval,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConfidenceInterval {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) lower_bound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) upper_bound: Option<String>,
}

impl From<ConfidenceInterval> for models::modelcard::ConfidenceInterval {
    fn from(other: ConfidenceInterval) -> Self {
        Self {
            lower_bound: convert_optional(other.lower_bound),
            upper_bound: convert_optional(other.upper_bound),
        }
    }
}

impl From<models::modelcard::ConfidenceInterval> for ConfidenceInterval {
    fn from(other: models::modelcard::ConfidenceInterval) -> Self {
        Self {
            lower_bound: convert_optional(other.lower_bound),
            upper_bound: convert_optional(other.upper_bound),
        }
    }
}

const LOWER_BOUND_TAG: &str = "lowerBound";
const UPPER_BOUND_TAG: &str = "upperBound";

impl ToXml for ConfidenceInterval {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, CONFIDENCE_INTERVAL_TAG)?;

        if let Some(lower_bound) = &self.lower_bound {
            write_simple_tag(writer, LOWER_BOUND_TAG, lower_bound)?;
        }

        if let Some(upper_bound) = &self.upper_bound {
            write_simple_tag(writer, UPPER_BOUND_TAG, upper_bound)?;
        }

        write_close_tag(writer, CONFIDENCE_INTERVAL_TAG)?;

        Ok(())
    }
}

impl FromXml for ConfidenceInterval {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut lower_bound: Option<String> = None;
        let mut upper_bound: Option<String> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement { name, .. }
                    if name.local_name == LOWER_BOUND_TAG =>
                {
                    lower_bound = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement { name, .. }
                    if name.local_name == UPPER_BOUND_TAG =>
                {
                    upper_bound = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self {
            lower_bound,
            upper_bound,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Considerations {}

impl From<models::modelcard::Considerations> for Considerations {
    fn from(_other: models::modelcard::Considerations) -> Self {
        Self {}
    }
}

impl From<Considerations> for models::modelcard::Considerations {
    fn from(_other: Considerations) -> Self {
        Self {}
    }
}

const CONSIDERATIONS_TAG: &str = "considerations";

impl ToXml for Considerations {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, CONSIDERATIONS_TAG)?;

        // TODO: implement

        write_close_tag(writer, CONSIDERATIONS_TAG)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Inputs(pub Vec<MLParameter>);

impl From<models::modelcard::Inputs> for Inputs {
    fn from(other: models::modelcard::Inputs) -> Self {
        Self(convert_vec(other.0))
    }
}

impl From<Inputs> for models::modelcard::Inputs {
    fn from(other: Inputs) -> Self {
        Self(convert_vec(other.0))
    }
}

impl ToXml for Inputs {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, INPUTS_TAG)?;

        for input in self.0.iter() {
            write_start_tag(writer, INPUT_TAG)?;
            input.write_xml_element(writer)?;
            write_close_tag(writer, INPUT_TAG)?;
        }

        write_close_tag(writer, INPUTS_TAG)?;

        Ok(())
    }
}

impl FromXml for Inputs {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &xml::name::OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut inputs: Vec<MLParameter> = Vec::new();

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == INPUT_TAG => {
                    let parameter =
                        MLParameter::read_xml_element(event_reader, &name, &attributes)?;
                    inputs.push(parameter);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self(inputs))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Outputs(pub Vec<MLParameter>);

impl From<models::modelcard::Outputs> for Outputs {
    fn from(other: models::modelcard::Outputs) -> Self {
        Self(convert_vec(other.0))
    }
}

impl From<Outputs> for models::modelcard::Outputs {
    fn from(other: Outputs) -> Self {
        Self(convert_vec(other.0))
    }
}

impl ToXml for Outputs {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, OUTPUTS_TAG)?;

        for output in self.0.iter() {
            write_start_tag(writer, OUTPUT_TAG)?;
            output.write_xml_element(writer)?;
            write_close_tag(writer, OUTPUT_TAG)?;
        }

        write_close_tag(writer, OUTPUTS_TAG)?;

        Ok(())
    }
}

impl FromXml for Outputs {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &xml::name::OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut outputs: Vec<MLParameter> = Vec::new();

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == OUTPUT_TAG => {
                    let parameter =
                        MLParameter::read_xml_element(event_reader, &name, &attributes)?;
                    outputs.push(parameter);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self(outputs))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct MLParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
}

impl MLParameter {
    #[allow(unused)]
    pub fn new(format: &str) -> Self {
        Self {
            format: Some(format.to_string()),
        }
    }
}

impl From<models::modelcard::MLParameter> for MLParameter {
    fn from(other: models::modelcard::MLParameter) -> Self {
        Self {
            format: convert_optional(other.format),
        }
    }
}

impl From<MLParameter> for models::modelcard::MLParameter {
    fn from(other: MLParameter) -> Self {
        Self {
            format: convert_optional(other.format),
        }
    }
}

impl ToXml for MLParameter {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        if let Some(format) = &self.format {
            write_simple_tag(writer, FORMAT_TAG, format)?;
        }

        Ok(())
    }
}

impl FromXml for MLParameter {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut format: Option<String> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader.next().map_err(to_xml_read_error(OUTPUT_TAG))?;
            match next_element {
                reader::XmlEvent::StartElement { name, .. } if name.local_name == FORMAT_TAG => {
                    format = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self { format })
    }
}

/// For more details see:
/// https://cyclonedx.org/docs/1.5/json/#components_items_modelCard_modelParameters_datasets_items_oneOf_i0_graphics
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Graphics {
    pub(crate) description: Option<String>,
    pub(crate) collection: Option<Collection>,
}

impl From<models::modelcard::Graphics> for Graphics {
    fn from(other: models::modelcard::Graphics) -> Self {
        Self {
            description: convert_optional(other.description),
            collection: convert_optional(other.collection),
        }
    }
}

impl From<Graphics> for models::modelcard::Graphics {
    fn from(other: Graphics) -> Self {
        Self {
            description: convert_optional(other.description),
            collection: convert_optional(other.collection),
        }
    }
}

const COLLECTION_TAG: &str = "collection";
const DESCRIPTION_TAG: &str = "description";

impl ToXml for Graphics {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, GRAPHICS_TAG)?;

        if let Some(description) = &self.description {
            write_simple_tag(writer, DESCRIPTION_TAG, description)?;
        }

        if let Some(collection) = &self.collection {
            collection.write_xml_element(writer)?;
        }

        write_close_tag(writer, GRAPHICS_TAG)?;

        Ok(())
    }
}

impl FromXml for Graphics {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut description: Option<String> = None;
        let mut collection: Option<Collection> = None;

        let mut got_end_tag = false;
        while !got_end_tag {
            let next_element = event_reader.next().map_err(to_xml_read_error(OUTPUT_TAG))?;
            match next_element {
                reader::XmlEvent::StartElement { name, .. }
                    if name.local_name == DESCRIPTION_TAG =>
                {
                    description = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == COLLECTION_TAG => {
                    collection = Some(Collection::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self {
            description,
            collection,
        })
    }
}

/// Helper struct to collect all [`Graphic`].
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Collection(pub(crate) Vec<Graphic>);

impl From<Vec<models::modelcard::Graphic>> for Collection {
    fn from(other: Vec<models::modelcard::Graphic>) -> Self {
        Self(convert_vec(other))
    }
}

impl From<Collection> for Vec<models::modelcard::Graphic> {
    fn from(other: Collection) -> Self {
        convert_vec(other.0)
    }
}

const GRAPHIC_TAG: &str = "graphic";

impl ToXml for Collection {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, COLLECTION_TAG)?;

        for graphic in &self.0 {
            graphic.write_xml_element(writer)?;
        }

        write_close_tag(writer, COLLECTION_TAG)?;

        Ok(())
    }
}

impl FromXml for Collection {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut collection: Vec<Graphic> = Vec::new();
        let mut got_end_tag = false;

        while !got_end_tag {
            let next_element = event_reader.next().map_err(to_xml_read_error(OUTPUT_TAG))?;

            match next_element {
                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == GRAPHIC_TAG => {
                    collection.push(Graphic::read_xml_element(event_reader, &name, &attributes)?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self(collection))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Graphic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) image: Option<Attachment>,
}

impl From<models::modelcard::Graphic> for Graphic {
    fn from(other: models::modelcard::Graphic) -> Self {
        Self {
            name: convert_optional(other.name),
            image: convert_optional(other.image),
        }
    }
}

impl From<Graphic> for models::modelcard::Graphic {
    fn from(other: Graphic) -> Self {
        Self {
            name: convert_optional(other.name),
            image: convert_optional(other.image),
        }
    }
}

const IMAGE_TAG: &str = "image";

impl ToXml for Graphic {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, GRAPHIC_TAG)?;

        if let Some(name) = &self.name {
            write_simple_tag(writer, NAME_TAG, name)?;
        }

        if let Some(image) = &self.image {
            image.write_xml_named_element(writer, IMAGE_TAG)?;
        }

        write_close_tag(writer, GRAPHIC_TAG)?;

        Ok(())
    }
}

impl FromXml for Graphic {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut graphic_name: Option<String> = None;
        let mut image: Option<Attachment> = None;

        let mut got_end_tag = false;

        while !got_end_tag {
            let next_element = event_reader.next().map_err(to_xml_read_error(OUTPUT_TAG))?;
            match next_element {
                reader::XmlEvent::StartElement { name, .. } if name.local_name == NAME_TAG => {
                    graphic_name = Some(read_simple_tag(event_reader, &name)?);
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == IMAGE_TAG => {
                    image = Some(Attachment::read_xml_element(
                        event_reader,
                        &name,
                        &attributes,
                    )?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self {
            name: graphic_name,
            image,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DataGovernance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) custodians: Option<Vec<DataGovernanceResponsibleParty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stewards: Option<Vec<DataGovernanceResponsibleParty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) owners: Option<Vec<DataGovernanceResponsibleParty>>,
}

impl From<models::modelcard::DataGovernance> for DataGovernance {
    fn from(other: models::modelcard::DataGovernance) -> Self {
        Self {
            custodians: other.custodians.map(convert_vec),
            stewards: other.stewards.map(convert_vec),
            owners: other.owners.map(convert_vec),
        }
    }
}

impl From<DataGovernance> for models::modelcard::DataGovernance {
    fn from(other: DataGovernance) -> Self {
        Self {
            custodians: other.custodians.map(convert_vec),
            stewards: other.stewards.map(convert_vec),
            owners: other.owners.map(convert_vec),
        }
    }
}

const CUSTODIANS_TAG: &str = "custodians";
const CUSTODIAN_TAG: &str = "custodian";
const STEWARDS_TAG: &str = "stewards";
const STEWARD_TAG: &str = "steward";
const OWNERS_TAG: &str = "owners";
const OWNER_TAG: &str = "owner";

impl ToXml for DataGovernance {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        write_start_tag(writer, GOVERNANCE_TAG)?;

        if let Some(owners) = &self.owners {
            write_start_tag(writer, OWNERS_TAG)?;
            for owner in owners {
                write_start_tag(writer, OWNER_TAG)?;
                owner.write_xml_element(writer)?;
                write_close_tag(writer, OWNER_TAG)?;
            }
            write_close_tag(writer, OWNERS_TAG)?;
        }

        if let Some(custodians) = &self.custodians {
            write_start_tag(writer, CUSTODIANS_TAG)?;
            for custodian in custodians {
                write_start_tag(writer, CUSTODIAN_TAG)?;
                custodian.write_xml_element(writer)?;
                write_close_tag(writer, CUSTODIAN_TAG)?;
            }
            write_close_tag(writer, CUSTODIANS_TAG)?;
        }

        if let Some(stewards) = &self.stewards {
            write_start_tag(writer, STEWARDS_TAG)?;
            for steward in stewards {
                write_start_tag(writer, STEWARD_TAG)?;
                steward.write_xml_element(writer)?;
                write_close_tag(writer, STEWARD_TAG)?;
            }
            write_close_tag(writer, STEWARDS_TAG)?;
        }

        write_close_tag(writer, GOVERNANCE_TAG)?;

        Ok(())
    }
}

impl FromXml for DataGovernance {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut custodians: Option<Vec<DataGovernanceResponsibleParty>> = None;
        let mut stewards: Option<Vec<DataGovernanceResponsibleParty>> = None;
        let mut owners: Option<Vec<DataGovernanceResponsibleParty>> = None;
        let mut got_end_tag = false;

        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement { name, .. }
                    if name.local_name == CUSTODIANS_TAG =>
                {
                    custodians = Some(read_list_tag(event_reader, &name, CUSTODIAN_TAG)?);
                }

                reader::XmlEvent::StartElement { name, .. } if name.local_name == STEWARDS_TAG => {
                    stewards = Some(read_list_tag(event_reader, &name, STEWARD_TAG)?);
                }

                reader::XmlEvent::StartElement { name, .. } if name.local_name == OWNERS_TAG => {
                    owners = Some(read_list_tag(event_reader, &name, OWNER_TAG)?);
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                _ => (),
            }
        }

        Ok(Self {
            custodians,
            stewards,
            owners,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) enum DataGovernanceResponsibleParty {
    Organization(OrganizationalEntity),
    Contact(OrganizationalContact),
}

impl From<models::modelcard::DataGovernanceResponsibleParty> for DataGovernanceResponsibleParty {
    fn from(other: models::modelcard::DataGovernanceResponsibleParty) -> Self {
        match other {
            models::modelcard::DataGovernanceResponsibleParty::Organization(organization) => {
                Self::Organization(organization.into())
            }
            models::modelcard::DataGovernanceResponsibleParty::Contact(contact) => {
                Self::Contact(contact.into())
            }
        }
    }
}

impl From<DataGovernanceResponsibleParty> for models::modelcard::DataGovernanceResponsibleParty {
    fn from(other: DataGovernanceResponsibleParty) -> Self {
        match other {
            DataGovernanceResponsibleParty::Organization(organization) => {
                Self::Organization(organization.into())
            }
            DataGovernanceResponsibleParty::Contact(contact) => Self::Contact(contact.into()),
        }
    }
}

const ORGANIZATION_TAG: &str = "organization";
const CONTACT_TAG: &str = "contact";

impl ToXml for DataGovernanceResponsibleParty {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        match self {
            DataGovernanceResponsibleParty::Organization(organization) => {
                organization.write_xml_named_element(writer, ORGANIZATION_TAG)?;
            }
            DataGovernanceResponsibleParty::Contact(contact) => {
                contact.write_xml_named_element(writer, CONTACT_TAG)?;
            }
        }

        Ok(())
    }
}

impl FromXml for DataGovernanceResponsibleParty {
    fn read_xml_element<R: std::io::Read>(
        event_reader: &mut xml::EventReader<R>,
        element_name: &OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        let mut party: Option<DataGovernanceResponsibleParty> = None;
        let mut got_end_tag = false;

        while !got_end_tag {
            let next_element = event_reader
                .next()
                .map_err(to_xml_read_error(&element_name.local_name))?;

            match next_element {
                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == ORGANIZATION_TAG => {
                    let organization =
                        OrganizationalEntity::read_xml_element(event_reader, &name, &attributes)?;
                    party = Some(DataGovernanceResponsibleParty::Organization(organization));
                }

                reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == CONTACT_TAG => {
                    let contact =
                        OrganizationalContact::read_xml_element(event_reader, &name, &attributes)?;
                    party = Some(DataGovernanceResponsibleParty::Contact(contact));
                }

                reader::XmlEvent::EndElement { name } if &name == element_name => {
                    got_end_tag = true;
                }

                unexpected => return Err(unexpected_element_error(element_name, unexpected)),
            }
        }

        let party = party.ok_or_else(|| XmlReadError::RequiredDataMissing {
            required_field: "organization or contact".to_string(),
            element: element_name.local_name.to_string(),
        })?;

        Ok(party)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use pretty_assertions::assert_eq;

    use crate::{
        models::{self, bom::BomReference},
        prelude::{NormalizedString, Uri},
        specs::{
            common::organization::{OrganizationalContact, OrganizationalEntity},
            v1_5::modelcard::{
                Attachment, Collection, ComponentData, ConfidenceInterval, DataContents,
                DataGovernance, DataGovernanceResponsibleParty, Dataset, Datasets, Graphic,
                Graphics, Inputs, MLParameter, ModelCard, ModelParameters, ModelParametersApproach,
                Outputs, PerformanceMetric, PerformanceMetrics, QuantitativeAnalysis,
            },
        },
        xml::test::{read_element_from_string, write_element_to_string},
    };

    pub(crate) fn example_modelcard() -> ModelCard {
        ModelCard {
            bom_ref: Some("modelcard-1".to_string()),
            model_parameters: Some(example_model_parameters()),
            quantitative_analysis: Some(super::QuantitativeAnalysis {
                performance_metrics: Some(PerformanceMetrics(vec![PerformanceMetric {
                    metric_type: Some("metric-1".to_string()),
                    value: Some("metric value".to_string()),
                    slice: None,
                    confidence_interval: Some(ConfidenceInterval {
                        lower_bound: Some("low".to_string()),
                        upper_bound: Some("high".to_string()),
                    }),
                }])),
                graphics: Some(Graphics {
                    description: Some("Graphic Desc".to_string()),
                    collection: Some(Collection(vec![Graphic {
                        name: Some("Graphic A".to_string()),
                        image: Some(Attachment {
                            content: "1234".to_string(),
                            content_type: None,
                            encoding: None,
                        }),
                    }])),
                }),
            }),
            considerations: None,
            properties: None,
        }
    }

    pub(crate) fn corresponding_modelcard() -> models::modelcard::ModelCard {
        models::modelcard::ModelCard {
            bom_ref: Some(BomReference::new("modelcard-1")),
            model_parameters: Some(corresponding_model_parameters()),
            quantitative_analysis: Some(models::modelcard::QuantitativeAnalysis {
                performance_metrics: Some(models::modelcard::PerformanceMetrics(vec![
                    models::modelcard::PerformanceMetric {
                        metric_type: Some("metric-1".to_string()),
                        value: Some("metric value".to_string()),
                        slice: None,
                        confidence_interval: Some(models::modelcard::ConfidenceInterval {
                            lower_bound: Some("low".to_string()),
                            upper_bound: Some("high".to_string()),
                        }),
                    },
                ])),
                graphics: Some(models::modelcard::Graphics {
                    description: Some("Graphic Desc".to_string()),
                    collection: Some(vec![models::modelcard::Graphic {
                        name: Some("Graphic A".to_string()),
                        image: Some(models::attachment::Attachment {
                            content: "1234".to_string(),
                            content_type: None,
                            encoding: None,
                        }),
                    }]),
                }),
            }),
            considerations: None,
            properties: None,
        }
    }

    pub(crate) fn example_governance() -> DataGovernance {
        DataGovernance {
            custodians: None,
            stewards: None,
            owners: Some(vec![DataGovernanceResponsibleParty::Contact(
                OrganizationalContact {
                    bom_ref: Some("contact-1".to_string()),
                    name: Some("Contact".to_string()),
                    email: Some("contact@example.com".to_string()),
                    phone: None,
                },
            )]),
        }
    }

    pub(crate) fn corresponding_governance() -> models::modelcard::DataGovernance {
        models::modelcard::DataGovernance {
            custodians: None,
            stewards: None,
            owners: Some(vec![
                models::modelcard::DataGovernanceResponsibleParty::Contact(
                    models::organization::OrganizationalContact {
                        bom_ref: Some(BomReference::new("contact-1")),
                        name: Some(NormalizedString::new("Contact")),
                        email: Some(NormalizedString::new("contact@example.com")),
                        phone: None,
                    },
                ),
            ]),
        }
    }

    pub(crate) fn example_model_parameters() -> ModelParameters {
        ModelParameters {
            approach: Some(ModelParametersApproach {
                approach_type: Some("supervised".to_string()),
            }),
            task: Some("Task".to_string()),
            architecture_family: Some("Architecture".to_string()),
            model_architecture: Some("Model".to_string()),
            datasets: Some(Datasets(vec![Dataset::Component(ComponentData {
                bom_ref: Some("dataset-1".to_string()),
                data_type: "dataset".to_string(),
                name: Some("Training Data".to_string()),
                contents: Some(DataContents {
                    attachment: None,
                    url: Some("https://example.com/path/to/dataset".to_string()),
                    properties: None,
                }),
                classification: Some("public".to_string()),
                sensitive_data: None,
                graphics: None,
                description: None,
                governance: Some(example_governance()),
            })])),
            inputs: Some(Inputs(vec![MLParameter::new("string")])),
            outputs: Some(Outputs(vec![MLParameter::new("image")])),
        }
    }

    pub(crate) fn corresponding_model_parameters() -> models::modelcard::ModelParameters {
        models::modelcard::ModelParameters {
            approach: Some(models::modelcard::ModelParametersApproach::new(
                "supervised",
            )),
            task: Some("Task".to_string()),
            architecture_family: Some("Architecture".to_string()),
            model_architecture: Some("Model".to_string()),
            datasets: Some(models::modelcard::Datasets(vec![
                models::modelcard::Dataset::Component(models::modelcard::ComponentData {
                    bom_ref: Some(BomReference::new("dataset-1")),
                    data_type: models::modelcard::ComponentDataType::Dataset,
                    name: Some("Training Data".to_string()),
                    contents: Some(models::modelcard::DataContents {
                        attachment: None,
                        url: Some(Uri("https://example.com/path/to/dataset".to_string())),
                        properties: None,
                    }),
                    classification: Some("public".to_string()),
                    sensitive_data: None,
                    graphics: None,
                    description: None,
                    governance: Some(corresponding_governance()),
                }),
            ])),
            inputs: Some(models::modelcard::Inputs(vec![
                models::modelcard::MLParameter::new("string"),
            ])),
            outputs: Some(models::modelcard::Outputs(vec![
                models::modelcard::MLParameter::new("image"),
            ])),
        }
    }

    #[test]
    fn it_should_write_xml_model_card() {
        let xml_output = write_element_to_string(example_modelcard());
        insta::assert_snapshot!(xml_output);
    }

    #[test]
    fn it_should_write_xml_data_governance() {
        let xml_output = write_element_to_string(example_governance());
        insta::assert_snapshot!(xml_output);
    }

    #[test]
    fn it_should_write_xml_model_parameters() {
        let xml_output = write_element_to_string(example_model_parameters());
        insta::assert_snapshot!(xml_output);
    }

    #[test]
    fn it_should_read_confidence_interval() {
        let input = r#"
<confidenceInterval>
  <lowerBound>The lower bound</lowerBound>
  <upperBound>The upper bound</upperBound>
</confidenceInterval>
"#;
        let actual: ConfidenceInterval = read_element_from_string(input);
        let expected = ConfidenceInterval {
            lower_bound: Some("The lower bound".to_string()),
            upper_bound: Some("The upper bound".to_string()),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_quantitative_analysis() {
        let input = r#"
<quantitativeAnalysis>
  <performanceMetrics>
    <performanceMetric>
      <type>The type of performance metric</type>
      <value>The value of the performance metric</value>
      <slice>The name of the slice this metric was computed on. By default, assume this metric is not sliced</slice>
      <confidenceInterval>
        <lowerBound>The lower bound of the confidence interval</lowerBound>
        <upperBound>The upper bound of the confidence interval</upperBound>
      </confidenceInterval>
    </performanceMetric>
  </performanceMetrics>
  <graphics>
    <description>Performance images</description>
    <collection>
      <graphic>
        <name>FID vs CLIP Scores on 512x512 samples for different v1-versions</name>
        <image encoding="base64" content-type="image/jpeg">1234</image>
      </graphic>
    </collection>
  </graphics>
</quantitativeAnalysis>
"#;
        let actual: QuantitativeAnalysis = read_element_from_string(input);
        let expected = QuantitativeAnalysis {
            performance_metrics: Some(PerformanceMetrics(vec![PerformanceMetric {
                metric_type: Some("The type of performance metric".to_string()),
                value: Some("The value of the performance metric".to_string()),
                slice: Some("The name of the slice this metric was computed on. By default, assume this metric is not sliced".to_string()),
                confidence_interval: Some(ConfidenceInterval {
                    lower_bound: Some("The lower bound of the confidence interval".to_string()),
                    upper_bound: Some("The upper bound of the confidence interval".to_string())
                })
            }])),
            graphics: Some(Graphics {
                description: Some("Performance images".to_string()),
                collection: Some(Collection(vec![Graphic {
                    name: Some(
                        "FID vs CLIP Scores on 512x512 samples for different v1-versions"
                            .to_string(),
                    ),
                    image: Some(Attachment {
                        content: "1234".to_string(),
                        content_type: Some("image/jpeg".to_string()),
                        encoding: Some("base64".to_string()),
                    }),
                }])),
            }),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_image_attachment() {
        let input = r#"
<image encoding="base64" content-type="image/jpeg">abcdefgh</image>
"#;
        let actual: Attachment = read_element_from_string(input);
        let expected = Attachment {
            content: "abcdefgh".to_string(),
            content_type: Some("image/jpeg".to_string()),
            encoding: Some("base64".to_string()),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_graphic() {
        let input = r#"
<graphic>
  <name>FID vs CLIP Scores on 512x512 samples for different v1-versions</name>
  <image encoding="base64" content-type="image/jpeg">abcdefgh</image>
</graphic>
"#;
        let actual: Graphic = read_element_from_string(input);
        let expected = Graphic {
            name: Some(
                "FID vs CLIP Scores on 512x512 samples for different v1-versions".to_string(),
            ),
            image: Some(Attachment {
                content: "abcdefgh".to_string(),
                content_type: Some("image/jpeg".to_string()),
                encoding: Some("base64".to_string()),
            }),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_graphics() {
        let input = r#"
<graphics>
  <description>Performance images</description>
  <collection>
    <graphic>
      <name>FID vs CLIP Scores on 512x512 samples for different v1-versions</name>
      <image encoding="base64" content-type="image/jpeg">abcdefgh</image>
    </graphic>
  </collection>
</graphics>
"#;
        let actual: Graphics = read_element_from_string(input);
        let expected = Graphics {
            description: Some("Performance images".to_string()),
            collection: Some(Collection(vec![Graphic {
                name: Some(
                    "FID vs CLIP Scores on 512x512 samples for different v1-versions".to_string(),
                ),
                image: Some(Attachment {
                    content: "abcdefgh".to_string(),
                    content_type: Some("image/jpeg".to_string()),
                    encoding: Some("base64".to_string()),
                }),
            }])),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_ml_parameter() {
        let input = r#"
<input>
  <format>string</format>
</input>
"#;
        let actual: MLParameter = read_element_from_string(input);
        let expected = MLParameter::new("string");
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_parse_xml_inputs() {
        let input = r#"
<inputs>
  <input>
    <format>string</format>
  </input>
  <input>
    <format>input</format>
  </input>
</inputs>
"#;
        let actual: Inputs = read_element_from_string(input);
        let expected = Inputs(vec![MLParameter::new("string"), MLParameter::new("input")]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_governance() {
        let input = r#"
<governance>
  <owners>
    <owner>
      <organization>
        <name>Organization 1</name>
      </organization>
    </owner>
  </owners>
  <custodians>
    <custodian>
      <contact bom-ref="custodian-1">
        <name>Custodian 1</name>
        <email>custodian@example.com</email>
      </contact>
    </custodian>
  </custodians>
</governance>
"#;
        let actual: DataGovernance = read_element_from_string(input);
        let expected = DataGovernance {
            custodians: Some(vec![DataGovernanceResponsibleParty::Contact(
                OrganizationalContact {
                    bom_ref: Some("custodian-1".to_string()),
                    name: Some("Custodian 1".to_string()),
                    email: Some("custodian@example.com".to_string()),
                    phone: None,
                },
            )]),
            stewards: None,
            owners: Some(vec![DataGovernanceResponsibleParty::Organization(
                OrganizationalEntity::new("Organization 1"),
            )]),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_dataset() {
        let input = r#"
<dataset bom-ref="dataset-a">
  <type>dataset</type>
  <name>Training Data</name>
  <contents>
    <url>https://example.com/path/to/dataset</url>
  </contents>
  <classification>public</classification>
  <description>data description</description>
  <governance>
    <owners>
      <owner>
        <organization>
          <name>Organization name</name>
        </organization>
      </owner>
    </owners>
  </governance>
</dataset>
"#;
        let actual: Dataset = read_element_from_string(input);
        let expected = Dataset::Component(ComponentData {
            bom_ref: Some("dataset-a".to_string()),
            data_type: "dataset".to_string(),
            name: Some("Training Data".to_string()),
            contents: Some(DataContents {
                attachment: None,
                url: Some("https://example.com/path/to/dataset".to_string()),
                properties: None,
            }),
            sensitive_data: None,
            classification: Some("public".to_string()),
            graphics: None,
            description: Some("data description".to_string()),
            governance: Some(DataGovernance {
                custodians: None,
                stewards: None,
                owners: Some(vec![DataGovernanceResponsibleParty::Organization(
                    OrganizationalEntity::new("Organization name"),
                )]),
            }),
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_json_datasets() {
        let input = r#"
[
  {
    "type": "dataset",
    "name": "Training Data",
    "contents": {
      "url": "https://example.com/path/to/dataset"
    },
    "classification": "public"
  }
]
"#;
        let actual: Datasets = serde_json::from_str(input).expect("Failed to parse JSON");
        let expected = Datasets(vec![Dataset::Component(ComponentData {
            bom_ref: None,
            data_type: "dataset".to_string(),
            name: Some("Training Data".to_string()),
            contents: Some(DataContents {
                attachment: None,
                url: Some("https://example.com/path/to/dataset".to_string()),
                properties: None,
            }),
            classification: Some("public".to_string()),
            sensitive_data: None,
            graphics: None,
            description: None,
            governance: None,
        })]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_model_parameters_approach() {
        let input = r#"
<approach>
  <type>supervised</type>
</approach>
"#;
        let actual: ModelParametersApproach = read_element_from_string(input);
        let expected = ModelParametersApproach {
            approach_type: Some("supervised".to_string()),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_model_parameters() {
        let input = r#"
<modelParameters>
  <approach>
    <type>supervised</type>
  </approach>
  <task>Task</task>
  <architectureFamily>Architecture</architectureFamily>
  <modelArchitecture>Model</modelArchitecture>
</modelParameters>
"#;
        let actual: ModelParameters = read_element_from_string(input);
        let expected = ModelParameters {
            approach: Some(ModelParametersApproach {
                approach_type: Some("supervised".to_string()),
            }),
            task: Some("Task".to_string()),
            architecture_family: Some("Architecture".to_string()),
            model_architecture: Some("Model".to_string()),
            datasets: None,
            inputs: None,
            outputs: None,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_sould_read_xml_model_card() {
        let input = r#"
<modelCard>
  <modelParameters>
    <approach>
      <type>supervised</type>
    </approach>
    <task>Task</task>
    <architectureFamily>Architecture</architectureFamily>
    <modelArchitecture>Model</modelArchitecture>
    <datasets>
      <dataset>
        <type>dataset</type>
        <name>Training Data</name>
        <contents>
          <url>https://example.com/path/to/dataset</url>
        </contents>
        <classification>public</classification>
      </dataset>
    </datasets>
    <inputs>
      <input><format>string</format></input>
    </inputs>
    <outputs>
      <output><format>image</format></output>
    </outputs>
  </modelParameters>
</modelCard>
"#;
        let actual: ModelCard = read_element_from_string(input);
        let expected = ModelCard {
            bom_ref: None,
            model_parameters: Some(ModelParameters {
                approach: Some(ModelParametersApproach {
                    approach_type: Some("supervised".to_string()),
                }),
                task: Some("Task".to_string()),
                architecture_family: Some("Architecture".to_string()),
                model_architecture: Some("Model".to_string()),
                datasets: Some(Datasets(vec![Dataset::Component(ComponentData {
                    bom_ref: None,
                    data_type: "dataset".to_string(),
                    name: Some("Training Data".to_string()),
                    contents: Some(DataContents {
                        attachment: None,
                        url: Some("https://example.com/path/to/dataset".to_string()),
                        properties: None,
                    }),
                    classification: Some("public".to_string()),
                    sensitive_data: None,
                    graphics: None,
                    description: None,
                    governance: None,
                })])),
                inputs: Some(Inputs(vec![MLParameter::new("string")])),
                outputs: Some(Outputs(vec![MLParameter::new("image")])),
            }),
            quantitative_analysis: None,
            considerations: None,
            properties: None,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_read_xml_bom_ref_attribute_in_modelcard() {
        let input = r#"
<modelCard bom-ref="modelcard-1">
</modelCard>
        "#;
        let actual: ModelCard = read_element_from_string(input);
        let expected = ModelCard {
            bom_ref: Some("modelcard-1".to_string()),
            model_parameters: None,
            quantitative_analysis: None,
            considerations: None,
            properties: None,
        };
        assert_eq!(expected, actual);
    }
}
