//! Module containing the definition of the SeededLweBoostrapKey.

use crate::core_crypto::algorithms::*;
use crate::core_crypto::commons::math::random::{ActivatedRandomGenerator, CompressionSeed};
use crate::core_crypto::commons::parameters::*;
use crate::core_crypto::commons::traits::*;
use crate::core_crypto::entities::*;

/// A [`seeded LWE bootstrap key`](`SeededLweBootstrapKey`).
///
/// This is a wrapper type of [`SeededGgswCiphertextList`], [`std::ops::Deref`] and
/// [`std::ops::DerefMut`] are implemented to dereference to the underlying
/// [`SeededGgswCiphertextList`] for ease of use. See [`SeededGgswCiphertextList`] for additional
/// methods.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SeededLweBootstrapKey<C: Container>
where
    C::Element: UnsignedInteger,
{
    // An SeededLweBootstrapKey is literally a SeededGgswCiphertextList, so we wrap a
    // GgswCiphertextList and use Deref to have access to all the primitives of the
    // SeededGgswCiphertextList easily
    ggsw_list: SeededGgswCiphertextList<C>,
}

impl<Scalar: UnsignedInteger, C: Container<Element = Scalar>> std::ops::Deref
    for SeededLweBootstrapKey<C>
{
    type Target = SeededGgswCiphertextList<C>;

    fn deref(&self) -> &SeededGgswCiphertextList<C> {
        &self.ggsw_list
    }
}

impl<Scalar: UnsignedInteger, C: ContainerMut<Element = Scalar>> std::ops::DerefMut
    for SeededLweBootstrapKey<C>
{
    fn deref_mut(&mut self) -> &mut SeededGgswCiphertextList<C> {
        &mut self.ggsw_list
    }
}

impl<Scalar: UnsignedInteger, C: Container<Element = Scalar>> SeededLweBootstrapKey<C> {
    /// Create an [`SeededLweBootstrapKey`] from an existing container.
    ///
    /// # Note
    ///
    /// This function only wraps a container in the appropriate type. If you want to generate an LWE
    /// bootstrap key you need to use
    /// [`crate::core_crypto::algorithms::generate_seeded_lwe_bootstrap_key`] or its parallel
    /// equivalent [`crate::core_crypto::algorithms::par_generate_seeded_lwe_bootstrap_key`]
    /// using this key as output.
    ///
    /// This docstring exhibits [`SeededLweBootstrapKey`] primitives usage.
    ///
    /// ```
    /// use tfhe::core_crypto::prelude::*;    
    ///
    /// // DISCLAIMER: these toy example parameters are not guaranteed to be secure or yield correct
    /// // computations
    /// // Define parameters for SeededLweBootstrapKey creation
    /// let glwe_size = GlweSize(2);
    /// let polynomial_size = PolynomialSize(1024);
    /// let decomp_base_log = DecompositionBaseLog(8);
    /// let decomp_level_count = DecompositionLevelCount(3);
    /// let input_lwe_dimension = LweDimension(600);
    ///
    /// // Get a seeder
    /// let mut seeder = new_seeder();
    /// let seeder = seeder.as_mut();
    ///
    /// // Create a new SeededLweBootstrapKey
    /// let bsk = SeededLweBootstrapKey::new(
    ///     0u64,
    ///     glwe_size,
    ///     polynomial_size,
    ///     decomp_base_log,
    ///     decomp_level_count,
    ///     input_lwe_dimension,
    ///     seeder.seed().into(),
    /// );
    ///
    /// // These methods are "inherited" from SeededGgswCiphertextList and are accessed through the Deref
    /// // trait
    /// assert_eq!(bsk.glwe_size(), glwe_size);
    /// assert_eq!(bsk.polynomial_size(), polynomial_size);
    /// assert_eq!(bsk.decomposition_base_log(), decomp_base_log);
    /// assert_eq!(bsk.decomposition_level_count(), decomp_level_count);
    ///
    /// // These methods are specific to the SeededLweBootstrapKey
    /// assert_eq!(bsk.input_lwe_dimension(), input_lwe_dimension);
    /// assert_eq!(
    ///     bsk.output_lwe_dimension().0,
    ///     glwe_size.to_glwe_dimension().0 * polynomial_size.0
    /// );
    ///
    /// let compression_seed = bsk.compression_seed();
    ///
    /// // Demonstrate how to recover the allocated container
    /// let underlying_container: Vec<u64> = bsk.into_container();
    ///
    /// // Recreate a key using from_container
    /// let bsk = SeededLweBootstrapKey::from_container(
    ///     underlying_container,
    ///     glwe_size,
    ///     polynomial_size,
    ///     decomp_base_log,
    ///     decomp_level_count,
    ///     compression_seed,
    /// );
    ///
    /// assert_eq!(bsk.glwe_size(), glwe_size);
    /// assert_eq!(bsk.polynomial_size(), polynomial_size);
    /// assert_eq!(bsk.decomposition_base_log(), decomp_base_log);
    /// assert_eq!(bsk.decomposition_level_count(), decomp_level_count);
    /// assert_eq!(bsk.input_lwe_dimension(), input_lwe_dimension);
    /// assert_eq!(
    ///     bsk.output_lwe_dimension().0,
    ///     glwe_size.to_glwe_dimension().0 * polynomial_size.0
    /// );
    ///
    /// let bsk = bsk.decompress_into_lwe_bootstrap_key();
    ///
    /// assert_eq!(bsk.glwe_size(), glwe_size);
    /// assert_eq!(bsk.polynomial_size(), polynomial_size);
    /// assert_eq!(bsk.decomposition_base_log(), decomp_base_log);
    /// assert_eq!(bsk.decomposition_level_count(), decomp_level_count);
    /// assert_eq!(bsk.input_lwe_dimension(), input_lwe_dimension);
    /// assert_eq!(
    ///     bsk.output_lwe_dimension().0,
    ///     glwe_size.to_glwe_dimension().0 * polynomial_size.0
    /// );
    /// ```
    pub fn from_container(
        container: C,
        glwe_size: GlweSize,
        polynomial_size: PolynomialSize,
        decomp_base_log: DecompositionBaseLog,
        decomp_level_count: DecompositionLevelCount,
        compression_seed: CompressionSeed,
        ciphertext_modulus: CiphertextModulus<C::Element>,
    ) -> SeededLweBootstrapKey<C> {
        SeededLweBootstrapKey {
            ggsw_list: SeededGgswCiphertextList::from_container(
                container,
                glwe_size,
                polynomial_size,
                decomp_base_log,
                decomp_level_count,
                compression_seed,
                ciphertext_modulus,
            ),
        }
    }

    /// Return the [`LweDimension`] of the input [`LweSecretKey`].
    ///
    /// See [`SeededLweBootstrapKey::from_container`] for usage.
    pub fn input_lwe_dimension(&self) -> LweDimension {
        LweDimension(self.ggsw_ciphertext_count().0)
    }

    /// Return the [`LweDimension`] of the equivalent output [`LweSecretKey`].
    ///
    /// See [`SeededLweBootstrapKey::from_container`] for usage.
    pub fn output_lwe_dimension(&self) -> LweDimension {
        LweDimension(self.glwe_size().to_glwe_dimension().0 * self.polynomial_size().0)
    }

    /// Consume the entity and return its underlying container.
    ///
    /// See [`SeededLweBootstrapKey::from_container`] for usage.
    pub fn into_container(self) -> C {
        self.ggsw_list.into_container()
    }

    /// Consume the [`SeededLweBootstrapKey`] and decompress it into a standard
    /// [`LweBootstrapKey`].
    ///
    /// See [`SeededLweBootstrapKey::from_container`] for usage.
    pub fn decompress_into_lwe_bootstrap_key(self) -> LweBootstrapKeyOwned<Scalar>
    where
        Scalar: UnsignedTorus,
    {
        let mut decompressed_bsk = LweBootstrapKeyOwned::new(
            Scalar::ZERO,
            self.glwe_size(),
            self.polynomial_size(),
            self.decomposition_base_log(),
            self.decomposition_level_count(),
            self.input_lwe_dimension(),
            self.ciphertext_modulus(),
        );
        decompress_seeded_lwe_bootstrap_key::<_, _, _, ActivatedRandomGenerator>(
            &mut decompressed_bsk,
            &self,
        );
        decompressed_bsk
    }

    /// Return a view of the [`SeededLweBootstrapKey`]. This is useful if an algorithm takes a view
    /// by value.
    pub fn as_view(&self) -> SeededLweBootstrapKey<&'_ [Scalar]> {
        SeededLweBootstrapKey::from_container(
            self.as_ref(),
            self.glwe_size(),
            self.polynomial_size(),
            self.decomposition_base_log(),
            self.decomposition_level_count(),
            self.compression_seed(),
            self.ciphertext_modulus(),
        )
    }
}

impl<Scalar: UnsignedInteger, C: ContainerMut<Element = Scalar>> SeededLweBootstrapKey<C> {
    /// Mutable variant of [`SeededLweBootstrapKey::as_view`].
    pub fn as_mut_view(&mut self) -> SeededLweBootstrapKey<&'_ mut [Scalar]> {
        let glwe_size = self.glwe_size();
        let polynomial_size = self.polynomial_size();
        let decomp_base_log = self.decomposition_base_log();
        let decomp_level_count = self.decomposition_level_count();
        let compression_seed = self.compression_seed();
        let ciphertext_modulus = self.ciphertext_modulus();
        SeededLweBootstrapKey::from_container(
            self.as_mut(),
            glwe_size,
            polynomial_size,
            decomp_base_log,
            decomp_level_count,
            compression_seed,
            ciphertext_modulus,
        )
    }
}

/// An [`SeededLweBootstrapKey`] owning the memory for its own storage.
pub type SeededLweBootstrapKeyOwned<Scalar> = SeededLweBootstrapKey<Vec<Scalar>>;

impl<Scalar: UnsignedInteger> SeededLweBootstrapKeyOwned<Scalar> {
    /// Allocate memory and create a new owned [`SeededLweBootstrapKey`].
    ///
    /// # Note
    ///
    /// This function allocates a vector of the appropriate size and wraps it in the appropriate
    /// type. If you want to generate an LWE bootstrap key you need to use
    /// [`crate::core_crypto::algorithms::generate_seeded_lwe_bootstrap_key`] or its parallel
    /// equivalent [`crate::core_crypto::algorithms::par_generate_seeded_lwe_bootstrap_key`] using
    /// this key as output.
    ///
    /// See [`SeededLweBootstrapKey::from_container`] for usage.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fill_with: Scalar,
        glwe_size: GlweSize,
        polynomial_size: PolynomialSize,
        decomp_base_log: DecompositionBaseLog,
        decomp_level_count: DecompositionLevelCount,
        input_lwe_dimension: LweDimension,
        compression_seed: CompressionSeed,
        ciphertext_modulus: CiphertextModulus<Scalar>,
    ) -> SeededLweBootstrapKeyOwned<Scalar> {
        SeededLweBootstrapKeyOwned {
            ggsw_list: SeededGgswCiphertextList::new(
                fill_with,
                glwe_size,
                polynomial_size,
                decomp_base_log,
                decomp_level_count,
                GgswCiphertextCount(input_lwe_dimension.0),
                compression_seed,
                ciphertext_modulus,
            ),
        }
    }
}
